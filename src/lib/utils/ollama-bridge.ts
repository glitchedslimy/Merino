// src/lib/utils/ollama-bridge.ts

import { Command } from '@tauri-apps/plugin-shell';
import { Ollama } from 'ollama/browser';

let ollamaClient: Ollama | null = null;

export async function startOllamaBridge() {
  const port = 3001; // The port you will use to communicate with the sidecar
  const host = '127.0.0.1';
  
  // Arguments to tell the sidecar to run as an API server
  const sidecar = Command.sidecar('binaries/ollama-bridge', [
    '--api',
    '--host',
    host,
    '--port',
    port.toString(),
  ]);

  const startPromise = new Promise<Ollama | null>((resolve, reject) => {
    let output = '';
    const timeout = setTimeout(() => {
      reject(new Error(`Sidecar failed to start in 15 seconds. Output was: ${output}`));
    }, 15000);

    sidecar.stdout.on('data', data => {
      output += data;
      console.log(`Sidecar (STDOUT): ${data}`);
      // The bridge should now log "LLMBridge API server listening on..."
      if (data.includes('LLMBridge API server listening on')) {
        clearTimeout(timeout);
        const ollamaClientInstance = new Ollama({ host: `http://${host}:${port}` });
        ollamaClient = ollamaClientInstance;
        console.log("OLLAMA_BRIDGE is now ready and connected.");
        resolve(ollamaClientInstance);
      }
    });

    sidecar.stderr.on('data', data => {
      output += data;
      console.error(`Sidecar (STDERR): ${data}`);
    });
    
    sidecar.on('close', data => {
      clearTimeout(timeout);
      console.log(`Sidecar process closed with code ${data.code}`);
      reject(new Error(`Sidecar process closed unexpectedly. Code: ${data.code}`));
    });

    sidecar.spawn().catch(err => {
      clearTimeout(timeout);
      reject(err);
    });
  });

  return startPromise;
}

export function getOllamaClient() {
  if (!ollamaClient) {
    throw new Error("Ollama client has not been initialized.");
  }
  return ollamaClient;
}