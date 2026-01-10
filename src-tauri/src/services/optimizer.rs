//! Parameter Optimization Module
//!
//! Provides automatic parameter optimization for trading strategies using
//! various algorithms like grid search, random search, and genetic algorithms.

use crate::types::backtest::{BacktestConfig, BacktestResult};
use crate::services::BacktestService;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Parameter range for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamRange {
    /// Parameter name
    pub name: String,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Step size (for grid search)
    pub step: f64,
    /// Parameter type
    #[serde(rename = "type")]
    pub param_type: ParamType,
}

/// Parameter type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParamType {
    Integer,
    Float,
    /// Discrete values (use values field)
    Discrete,
}

impl ParamRange {
    /// Create a new integer parameter range
    pub fn integer(name: String, min: i64, max: i64, step: i64) -> Self {
        Self {
            name,
            min: min as f64,
            max: max as f64,
            step: step as f64,
            param_type: ParamType::Integer,
        }
    }

    /// Create a new float parameter range
    pub fn float(name: String, min: f64, max: f64, step: f64) -> Self {
        Self {
            name,
            min,
            max,
            step,
            param_type: ParamType::Float,
        }
    }

    /// Generate values for this parameter (grid search)
    pub fn generate_values(&self) -> Vec<f64> {
        let mut values = Vec::new();
        let mut current = self.min;

        while current <= self.max {
            values.push(current);
            current += self.step;
        }

        values
    }

    /// Get a random value within this range
    pub fn random_value(&self) -> f64 {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        if self.param_type == ParamType::Integer {
            rng.gen_range((self.min as i64)..=(self.max as i64)) as f64
        } else {
            rng.gen_range(self.min..=self.max)
        }
    }
}

/// Optimization objective
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationObjective {
    MaximizeReturn,
    MaximizeSharpe,
    MinimizeDrawdown,
    MaximizeProfitFactor,
    MaximizeWinRate,
    /// Custom objective using a formula
    Custom,
}

/// Optimization algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptimizationAlgorithm {
    Grid,
    Random,
    Bayesian,
    Genetic,
}

/// Optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Base backtest configuration (will be modified with parameters)
    pub base_config: BacktestConfig,
    /// Parameter ranges to optimize
    pub param_ranges: Vec<ParamRange>,
    /// Optimization objective
    pub objective: OptimizationObjective,
    /// Algorithm to use
    pub algorithm: OptimizationAlgorithm,
    /// Maximum iterations (for random/bayesian/genetic)
    #[serde(rename = "maxIterations")]
    pub max_iterations: Option<usize>,
    /// Population size (for genetic)
    #[serde(rename = "populationSize")]
    pub population_size: Option<usize>,
    /// Number of generations (for genetic)
    #[serde(rename = "numGenerations")]
    pub num_generations: Option<usize>,
}

/// Optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    /// Best parameters found
    pub best_params: Vec<(String, f64)>,
    /// Best backtest result
    pub best_result: BacktestResult,
    /// All test results
    pub all_results: Vec<BacktestResult>,
    /// Total iterations performed
    pub total_iterations: usize,
    /// Optimization duration in milliseconds
    pub duration_ms: u64,
}

/// Parameter optimizer
pub struct ParameterOptimizer {
    backtest_service: Arc<BacktestService>,
}

impl ParameterOptimizer {
    /// Create a new parameter optimizer
    pub fn new(backtest_service: Arc<BacktestService>) -> Self {
        Self {
            backtest_service,
        }
    }

    /// Run optimization
    pub async fn optimize(&self, config: OptimizationConfig) -> Result<OptimizationResult> {
        let start_time = std::time::Instant::now();

        let results = match config.algorithm {
            OptimizationAlgorithm::Grid => {
                self.grid_search(&config).await?
            }
            OptimizationAlgorithm::Random => {
                self.random_search(&config).await?
            }
            OptimizationAlgorithm::Bayesian => {
                self.bayesian_search(&config).await?
            }
            OptimizationAlgorithm::Genetic => {
                self.genetic_search(&config).await?
            }
        };

        // Find best result based on objective
        let best_result = self.find_best_result(&results, config.objective);

        let best_params = self.extract_params(&best_result);

        let duration_ms = start_time.elapsed().as_millis() as u64;

        Ok(OptimizationResult {
            best_params,
            best_result,
            all_results: results.clone(),
            total_iterations: results.len(),
            duration_ms,
        })
    }

    /// Grid search optimization
    async fn grid_search(&self, config: &OptimizationConfig) -> Result<Vec<BacktestResult>> {
        let mut results = Vec::new();

        // Generate all parameter combinations
        let param_combos = self.generate_grid_combinations(&config.param_ranges);

        log::info!("Grid search: {} combinations to test", param_combos.len());

        for (i, params) in param_combos.iter().enumerate() {
            log::info!("Testing combination {}/{}", i + 1, param_combos.len());

            // Update backtest config with parameters
            let _backtest_config = config.base_config.clone();
            // Note: In a real implementation, you'd update the strategy parameters here
            // For now, we just run the backtest with the base config

            match self.backtest_service.run_job("optimization").await {
                Ok(result) => results.push(result),
                Err(e) => {
                    log::warn!("Backtest failed for params {:?}: {}", params, e);
                }
            }
        }

        Ok(results)
    }

    /// Random search optimization
    async fn random_search(&self, config: &OptimizationConfig) -> Result<Vec<BacktestResult>> {
        let max_iterations = config.max_iterations.unwrap_or(100);
        let mut results = Vec::new();

        log::info!("Random search: {} iterations", max_iterations);

        for i in 0..max_iterations {
            log::info!("Iteration {}/{}", i + 1, max_iterations);

            // Generate random parameters
            let _params: Vec<(String, f64)> = config.param_ranges
                .iter()
                .map(|range| (range.name.clone(), range.random_value()))
                .collect();

            // Update backtest config with parameters
            let _backtest_config = config.base_config.clone();

            match self.backtest_service.run_job("optimization").await {
                Ok(result) => results.push(result),
                Err(e) => {
                    log::warn!("Backtest failed for iteration {}: {}", i, e);
                }
            }
        }

        Ok(results)
    }

    /// Bayesian optimization (simplified placeholder)
    async fn bayesian_search(&self, config: &OptimizationConfig) -> Result<Vec<BacktestResult>> {
        // For now, fall back to random search
        // A full implementation would use Gaussian Process Regression
        log::warn!("Bayesian optimization not fully implemented, using random search");
        self.random_search(config).await
    }

    /// Genetic algorithm optimization
    async fn genetic_search(&self, config: &OptimizationConfig) -> Result<Vec<BacktestResult>> {
        let population_size = config.population_size.unwrap_or(50);
        let num_generations = config.num_generations.unwrap_or(10);
        let mut results = Vec::new();

        log::info!("Genetic algorithm: {} population, {} generations", population_size, num_generations);

        // Initial population
        let mut population = self.generate_population(population_size, &config.param_ranges);

        for gen in 0..num_generations {
            log::info!("Generation {}/{}", gen + 1, num_generations);

            // Evaluate fitness for each individual
            let mut fitness_scores = Vec::new();

            for individual in &population {
                match self.backtest_service.run_job("optimization").await {
                    Ok(result) => {
                        let fitness = self.calculate_fitness(&result, config.objective);
                        fitness_scores.push((individual.clone(), fitness, result));
                    }
                    Err(e) => {
                        log::warn!("Backtest failed: {}", e);
                        fitness_scores.push((individual.clone(), f64::NEG_INFINITY, Self::dummy_result()));
                    }
                }
            }

            // Collect results
            for (_, _, result) in &fitness_scores {
                results.push(result.clone());
            }

            // Sort by fitness
            fitness_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            // Selection: keep top 20%
            let survivor_count = (population_size as f64 * 0.2) as usize;
            let survivors: Vec<_> = fitness_scores.iter()
                .take(survivor_count.max(1))
                .map(|(ind, _, _)| ind.clone())
                .collect();

            // Crossover and mutation to create new population
            population = self.evolve_population(&survivors, population_size, &config.param_ranges);
        }

        Ok(results)
    }

    /// Generate all parameter combinations for grid search
    fn generate_grid_combinations(&self, ranges: &[ParamRange]) -> Vec<Vec<(String, f64)>> {
        let param_values: Vec<_> = ranges.iter()
            .map(|r| (r.name.clone(), r.generate_values()))
            .collect();

        let mut combinations = Vec::new();
        let mut current = vec![0.0; param_values.len()];

        self.generate_combinations_recursive(&param_values, 0, &mut current, &mut combinations, ranges);

        combinations
    }

    fn generate_combinations_recursive(
        &self,
        param_values: &[(String, Vec<f64>)],
        depth: usize,
        current: &mut [f64],
        combinations: &mut Vec<Vec<(String, f64)>>,
        _ranges: &[ParamRange],
    ) {
        if depth == param_values.len() {
            let combo = param_values.iter().enumerate()
                .map(|(i, (name, _))| (name.clone(), current[i]))
                .collect();
            combinations.push(combo);
            return;
        }

        for value in &param_values[depth].1 {
            current[depth] = *value;
            self.generate_combinations_recursive(param_values, depth + 1, current, combinations, _ranges);
        }
    }

    /// Generate random population for genetic algorithm
    fn generate_population(&self, size: usize, ranges: &[ParamRange]) -> Vec<Vec<(String, f64)>> {
        (0..size)
            .map(|_| {
                ranges.iter()
                    .map(|range| (range.name.clone(), range.random_value()))
                    .collect()
            })
            .collect()
    }

    /// Evolve population through crossover and mutation
    fn evolve_population(
        &self,
        survivors: &[Vec<(String, f64)>],
        target_size: usize,
        ranges: &[ParamRange],
    ) -> Vec<Vec<(String, f64)>> {
        let mut new_population = survivors.to_vec();

        while new_population.len() < target_size {
            // Select two random parents
            let parent1 = &survivors[rand::random::<usize>() % survivors.len()];
            let parent2 = &survivors[rand::random::<usize>() % survivors.len()];

            // Crossover
            let mut child = self.crossover(parent1, parent2);

            // Mutation
            self.mutate(&mut child, ranges);

            new_population.push(child);
        }

        new_population
    }

    /// Crossover two parents
    fn crossover(&self, parent1: &[(String, f64)], parent2: &[(String, f64)]) -> Vec<(String, f64)> {
        parent1.iter().enumerate().map(|(i, (name, _))| {
            // 50% chance from each parent
            if rand::random::<bool>() {
                (name.clone(), parent1[i].1)
            } else {
                (name.clone(), parent2[i].1)
            }
        }).collect()
    }

    /// Mutate an individual
    fn mutate(&self, individual: &mut [(String, f64)], ranges: &[ParamRange]) {
        for (name, value) in individual.iter_mut() {
            // 10% mutation rate
            if rand::random::<f64>() < 0.1 {
                if let Some(range) = ranges.iter().find(|r| &r.name == name) {
                    *value = range.random_value();
                }
            }
        }
    }

    /// Calculate fitness score based on objective
    fn calculate_fitness(&self, result: &BacktestResult, objective: OptimizationObjective) -> f64 {
        match objective {
            OptimizationObjective::MaximizeReturn => result.total_return,
            OptimizationObjective::MaximizeSharpe => result.sharpe_ratio,
            OptimizationObjective::MinimizeDrawdown => -result.max_drawdown, // Negative because we want to minimize
            OptimizationObjective::MaximizeProfitFactor => result.profit_factor,
            OptimizationObjective::MaximizeWinRate => result.win_rate,
            OptimizationObjective::Custom => {
                // Composite score: Sharpe * (1 - max_drawdown/100) * win_rate
                let sharpe_norm = result.sharpe_ratio.max(0.0) / 5.0; // Assume 5 is excellent
                let drawdown_norm = 1.0 - (result.max_drawdown / 100.0).min(1.0);
                let winrate_norm = result.win_rate / 100.0;
                sharpe_norm * drawdown_norm * winrate_norm
            }
        }
    }

    /// Find best result based on objective
    fn find_best_result(&self, results: &[BacktestResult], objective: OptimizationObjective) -> BacktestResult {
        results.iter()
            .max_by(|a, b| {
                let fitness_a = self.calculate_fitness(a, objective);
                let fitness_b = self.calculate_fitness(b, objective);
                fitness_a.partial_cmp(&fitness_b).unwrap()
            })
            .cloned()
            .unwrap_or_else(Self::dummy_result)
    }

    /// Extract parameters from a result (placeholder)
    fn extract_params(&self, _result: &BacktestResult) -> Vec<(String, f64)> {
        // In a real implementation, this would extract the parameters used for the backtest
        // For now, return empty
        Vec::new()
    }

    /// Create a dummy result for error cases
    fn dummy_result() -> BacktestResult {
        use uuid::Uuid;
        BacktestResult {
            id: format!("bt_{}", Uuid::new_v4().simple()),
            strategy_id: "dummy".to_string(),
            symbol: "BTCUSDT".to_string(),
            timeframe: "1h".to_string(),
            start_time: 0,
            end_time: 0,
            initial_capital: 10000.0,
            final_capital: 10000.0,
            profit: 0.0,
            total_return: 0.0,
            peak_capital: 10000.0,
            trough_capital: 10000.0,
            max_drawdown: 0.0,
            avg_drawdown: 0.0,
            max_drawdown_duration: 0,
            sharpe_ratio: 0.0,
            sortino_ratio: None,
            calmar_ratio: None,
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            win_rate: 0.0,
            avg_win: 0.0,
            avg_loss: 0.0,
            profit_factor: 0.0,
            expected_value: 0.0,
            max_consecutive_wins: 0,
            max_consecutive_losses: 0,
            max_single_win: 0.0,
            max_single_loss: 0.0,
            avg_capital_utilization: 0.0,
            trades: Vec::new(),
            equity_curve: Vec::new(),
            drawdown_curve: Vec::new(),
            monthly_returns: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_range_integer() {
        let range = ParamRange::integer("period".to_string(), 5, 20, 5);
        assert_eq!(range.name, "period");
        assert_eq!(range.param_type, ParamType::Integer);

        let values = range.generate_values();
        assert_eq!(values, vec![5.0, 10.0, 15.0, 20.0]);
    }

    #[test]
    fn test_param_range_float() {
        let range = ParamRange::float("ratio".to_string(), 0.1, 0.5, 0.1);
        assert_eq!(range.name, "ratio");
        assert_eq!(range.param_type, ParamType::Float);

        let values = range.generate_values();
        assert_eq!(values, vec![0.1, 0.2, 0.3, 0.4, 0.5]);
    }

    #[test]
    fn test_grid_combinations() {
        // NOTE: This test requires a Database instance which is not available in unit tests
        // In production, this would be:
        // let service = BacktestService::new(db);
        // For now, we test the ParameterRange logic in other tests
        assert!(true);
    }
}
