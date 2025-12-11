<script setup lang="ts" vapor>
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface SaveResult {
  needs_restart: boolean;
}

const props = defineProps<{
  currentShortcut: string;
  provider: 'openai' | 'mistral';
  language: string | null;
  openaiApiKey: string;
  mistralApiKey: string;
}>();

const emit = defineEmits<{
  'update:provider': [value: 'openai' | 'mistral'];
  'update:language': [value: string | null];
  'update:openaiApiKey': [value: string];
  'update:mistralApiKey': [value: string];
}>();

const openaiKeyMasked = ref(true);
const mistralKeyMasked = ref(true);
const status = ref("");

// Common language codes for the dropdown
const languageOptions = [
  { value: null, label: 'Auto-detect' },
  { value: 'en', label: 'English (en)' },
  { value: 'de', label: 'German (de)' },
  { value: 'fr', label: 'French (fr)' },
  { value: 'es', label: 'Spanish (es)' },
  { value: 'it', label: 'Italian (it)' },
  { value: 'pt', label: 'Portuguese (pt)' },
  { value: 'nl', label: 'Dutch (nl)' },
  { value: 'pl', label: 'Polish (pl)' },
  { value: 'ru', label: 'Russian (ru)' },
  { value: 'ja', label: 'Japanese (ja)' },
  { value: 'ko', label: 'Korean (ko)' },
  { value: 'zh', label: 'Chinese (zh)' },
];

const currentApiKeyConfigured = computed(() => {
  if (props.provider === 'openai') {
    return props.openaiApiKey.length > 0;
  }
  return props.mistralApiKey.length > 0;
});

async function saveSettings() {
  try {
    // Validate OpenAI key format if provided
    if (props.openaiApiKey && !props.openaiApiKey.startsWith('sk-')) {
      status.value = "Invalid OpenAI key format. Keys start with 'sk-'";
      return;
    }

    await invoke<SaveResult>('save_settings', {
      settings: {
        shortcut: props.currentShortcut,
        provider: props.provider,
        language: props.language,
        openai_api_key: props.openaiApiKey || null,
        mistral_api_key: props.mistralApiKey || null
      }
    });
    status.value = "Saved";
    setTimeout(() => status.value = "", 2000);
  } catch (e) {
    status.value = "Failed to save: " + e;
  }
}

function handleLanguageChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value;
  emit('update:language', value === '' ? null : value);
}
</script>

<template>
  <section class="section">
    <header class="section-header">
      <h1>Settings</h1>
      <p>Configure transcription provider and API keys</p>
    </header>

    <div class="section-content">
      <!-- Provider Selection -->
      <div class="field">
        <label>Transcription Provider</label>
        <div class="provider-options">
          <button
            class="provider-btn"
            :class="{ active: provider === 'openai' }"
            @click="emit('update:provider', 'openai')"
          >
            OpenAI Whisper
          </button>
          <button
            class="provider-btn"
            :class="{ active: provider === 'mistral' }"
            @click="emit('update:provider', 'mistral')"
          >
            Mistral Voxtral
          </button>
        </div>
        <p class="hint">
          {{ provider === 'openai' ? '~$0.006/minute' : '~$0.02/minute' }} · 
          {{ provider === 'openai' ? 'whisper-1 model' : 'voxtral-mini-latest model' }}
        </p>
      </div>

      <!-- Language Hint -->
      <div class="field">
        <label>Language Hint</label>
        <select
          class="select-input"
          :value="language ?? ''"
          @change="handleLanguageChange"
        >
          <option v-for="opt in languageOptions" :key="opt.value ?? 'auto'" :value="opt.value ?? ''">
            {{ opt.label }}
          </option>
        </select>
        <p class="hint">
          Helps improve accuracy. Leave on auto-detect if you speak multiple languages.
        </p>
      </div>

      <div class="divider"></div>

      <!-- OpenAI API Key -->
      <div class="field">
        <label>
          OpenAI API Key
          <span v-if="provider === 'openai'" class="active-badge">active</span>
        </label>
        <div class="api-key-input">
          <input
            :type="openaiKeyMasked ? 'password' : 'text'"
            :value="openaiApiKey"
            @input="emit('update:openaiApiKey', ($event.target as HTMLInputElement).value)"
            placeholder="sk-..."
            spellcheck="false"
            autocomplete="off"
          />
          <button @click="openaiKeyMasked = !openaiKeyMasked" class="toggle-btn" type="button">
            {{ openaiKeyMasked ? 'show' : 'hide' }}
          </button>
        </div>
        <p class="hint">
          Get your key from
          <a href="https://platform.openai.com/api-keys" target="_blank">platform.openai.com</a>
        </p>
      </div>

      <!-- Mistral API Key -->
      <div class="field">
        <label>
          Mistral API Key
          <span v-if="provider === 'mistral'" class="active-badge">active</span>
        </label>
        <div class="api-key-input">
          <input
            :type="mistralKeyMasked ? 'password' : 'text'"
            :value="mistralApiKey"
            @input="emit('update:mistralApiKey', ($event.target as HTMLInputElement).value)"
            placeholder="..."
            spellcheck="false"
            autocomplete="off"
          />
          <button @click="mistralKeyMasked = !mistralKeyMasked" class="toggle-btn" type="button">
            {{ mistralKeyMasked ? 'show' : 'hide' }}
          </button>
        </div>
        <p class="hint">
          Get your key from
          <a href="https://console.mistral.ai/api-keys" target="_blank">console.mistral.ai</a>
        </p>
      </div>

      <button @click="saveSettings" class="btn btn-secondary">Save</button>

      <div class="status" :class="{ visible: status }">{{ status }}</div>

      <div v-if="!currentApiKeyConfigured" class="notice">
        <span class="notice-marker">[!]</span>
        <p>Add your {{ provider === 'openai' ? 'OpenAI' : 'Mistral' }} API key to start transcribing.</p>
      </div>

      <div class="notice">
        <span class="notice-marker">[i]</span>
        <p>Settings stored locally in ~/.config/whis/settings.json</p>
      </div>
    </div>
  </section>
</template>

<style scoped>
/* Provider selection buttons */
.provider-options {
  display: flex;
  gap: 8px;
}

.provider-btn {
  flex: 1;
  padding: 10px 16px;
  background: var(--bg-weak);
  border: 1px solid var(--border);
  border-radius: 4px;
  font-family: var(--font);
  font-size: 12px;
  color: var(--text-weak);
  cursor: pointer;
  transition: all 0.15s ease;
}

.provider-btn:hover {
  border-color: var(--text-weak);
  color: var(--text);
}

.provider-btn.active {
  border-color: var(--accent);
  color: var(--accent);
  background: rgba(255, 213, 79, 0.1);
}

/* Select input */
.select-input {
  padding: 10px 12px;
  background: var(--bg-weak);
  border: 1px solid var(--border);
  border-radius: 4px;
  font-family: var(--font);
  font-size: 12px;
  color: var(--text);
  cursor: pointer;
  transition: border-color 0.15s ease;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23808080' d='M3 4.5L6 7.5L9 4.5'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;
  padding-right: 32px;
}

.select-input:focus {
  outline: none;
  border-color: var(--accent);
}

.select-input option {
  background: var(--bg);
  color: var(--text);
}

/* Divider */
.divider {
  height: 1px;
  background: var(--border);
  margin: 8px 0;
}

/* Active badge */
.active-badge {
  display: inline-block;
  padding: 2px 6px;
  margin-left: 8px;
  font-size: 9px;
  text-transform: uppercase;
  color: var(--accent);
  background: rgba(255, 213, 79, 0.15);
  border-radius: 3px;
  vertical-align: middle;
}

/* API key input */
.api-key-input {
  display: flex;
  gap: 8px;
}

.api-key-input input {
  flex: 1;
  padding: 10px 12px;
  background: var(--bg-weak);
  border: 1px solid var(--border);
  border-radius: 4px;
  font-family: var(--font);
  font-size: 12px;
  color: var(--text);
  transition: border-color 0.15s ease;
}

.api-key-input input::placeholder {
  color: var(--text-weak);
}

.api-key-input input:focus {
  outline: none;
  border-color: var(--accent);
}

.toggle-btn {
  padding: 10px 12px;
  background: var(--bg-weak);
  border: 1px solid var(--border);
  border-radius: 4px;
  font-family: var(--font);
  font-size: 11px;
  color: var(--text-weak);
  cursor: pointer;
  transition: all 0.15s ease;
}

.toggle-btn:hover {
  border-color: var(--text-weak);
  color: var(--text);
}
</style>
