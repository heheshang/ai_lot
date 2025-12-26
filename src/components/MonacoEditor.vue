<template>
  <div class="monaco-editor-container">
    <div ref="editorRef" class="editor"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, toRaw } from 'vue';
import type * as Monaco from 'monaco-editor';
import loader from '@monaco-editor/loader';

/**
 * Monaco Editor component props
 */
interface Props {
  modelValue: string;
  language?: string;
  theme?: 'vs' | 'vs-dark' | 'hc-black';
  readOnly?: boolean;
  fontSize?: number;
  minimap?: boolean;
  wordWrap?: 'on' | 'off' | 'wordWrapColumn' | 'bounded';
  lineNumbers?: 'on' | 'off' | 'relative' | 'interval';
}

const props = withDefaults(defineProps<Props>(), {
  language: 'javascript',
  theme: 'vs-dark',
  readOnly: false,
  fontSize: 14,
  minimap: true,
  wordWrap: 'off',
  lineNumbers: 'on',
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'ready': [editor: Monaco.editor.IStandaloneCodeEditor];
}>();

// Refs
const editorRef = ref<HTMLElement>();
let editor: Monaco.editor.IStandaloneCodeEditor | null = null;
let resizeObserver: ResizeObserver | null = null;

/**
 * Initialize Monaco Editor
 */
async function initEditor() {
  if (!editorRef.value) return;

  // Configure loader
  loader.config({
    paths: {
      vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.55.1/min/vs',
    },
  });

  // Initialize monaco
  const monaco = await loader.init();

  // Create editor instance
  const editorInstance = monaco.editor.create(editorRef.value, {
    value: props.modelValue,
    language: props.language,
    theme: props.theme,
    readOnly: props.readOnly,
    automaticLayout: false, // We'll handle layout manually
    minimap: {
      enabled: props.minimap,
    },
    fontSize: props.fontSize,
    scrollBeyondLastLine: false,
    roundedSelection: true,
    padding: { top: 16 },
    wordWrap: props.wordWrap,
    lineNumbers: props.lineNumbers,
    renderLineHighlight: 'all',
    cursorBlinking: 'smooth',
    cursorSmoothCaretAnimation: 'on',
    smoothScrolling: true,
    contextmenu: true,
    formatOnPaste: true,
    formatOnType: true,
    autoIndent: 'full',
    suggestOnTriggerCharacters: true,
    acceptSuggestionOnCommitCharacter: true,
    acceptSuggestionOnEnter: 'on',
    tabCompletion: 'on',
    folding: true,
    foldingStrategy: 'indentation',
    showFoldingControls: 'always',
    matchBrackets: 'always',
    bracketPairColorization: {
      enabled: true,
    },
    guides: {
      bracketPairs: true,
      indentation: true,
    },
  });

  // Store editor reference
  editor = editorInstance;

  // Content change handler
  editorInstance.onDidChangeModelContent(() => {
    const value = editorInstance.getValue();
    if (value !== undefined) {
      emit('update:modelValue', value);
    }
  });

  // Layout change handler
  editorInstance.onDidLayoutChange(() => {
    editorInstance.layout();
  });

  // Set up resize observer for responsive layout
  resizeObserver = new ResizeObserver(() => {
    editorInstance.layout();
  });
  resizeObserver.observe(editorRef.value);

  // Emit ready event
  emit('ready', toRaw(editorInstance));
}

/**
 * Update editor options
 */
function updateOptions() {
  if (!editor) return;

  editor.updateOptions({
    readOnly: props.readOnly,
    fontSize: props.fontSize,
    minimap: {
      enabled: props.minimap,
    },
    wordWrap: props.wordWrap,
    lineNumbers: props.lineNumbers,
  });
}

/**
 * Update language
 */
async function updateLanguage() {
  if (!editor) return;
  const monaco = await loader.init();
  const model = editor.getModel();
  if (model) {
    monaco.editor.setModelLanguage(model, props.language);
  }
}

/**
 * Update theme
 */
async function updateTheme() {
  if (!editor) return;
  const monaco = await loader.init();
  monaco.editor.setTheme(props.theme);
}

// Lifecycle
onMounted(async () => {
  await initEditor();
});

onUnmounted(() => {
  if (resizeObserver) {
    resizeObserver.disconnect();
  }
  if (editor) {
    editor.dispose();
    editor = null;
  }
});

// Watch for prop changes
watch(() => props.modelValue, (newValue) => {
  if (editor && newValue !== editor.getValue()) {
    const position = editor.getPosition();
    editor.setValue(newValue);
    if (position) {
      editor.setPosition(position);
    }
  }
});

watch(
  () => [props.fontSize, props.minimap, props.wordWrap, props.lineNumbers, props.readOnly] as const,
  () => {
    updateOptions();
  }
);

watch(() => props.language, () => {
  updateLanguage();
});

watch(() => props.theme, () => {
  updateTheme();
});

// Expose methods for parent components
defineExpose({
  getEditor: () => editor,
  getValue: () => editor?.getValue() ?? '',
  setValue: (value: string) => editor?.setValue(value),
  getSelection: () => editor?.getSelection(),
  setSelection: (selection: Monaco.Selection) => editor?.setSelection(selection),
  focus: () => editor?.focus(),
  layout: () => editor?.layout(),
});
</script>

<style scoped>
.monaco-editor-container {
  height: 100%;
  width: 100%;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  overflow: hidden;
}

.editor {
  height: 100%;
  width: 100%;
  min-height: 200px;
}
</style>
