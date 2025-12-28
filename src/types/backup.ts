// ============== Backup Types ==============

export interface BackupInfo {
  path: string;
  size: number;
  createdAt: number;
  compressed: boolean;
}

export interface BackupCreateResponse {
  path: string;
  size: number;
  createdAt: number;
  compressed: boolean;
}

export interface BackupListResponse {
  backups: BackupInfo[];
  totalSize: number;
  totalBackups: number;
}
