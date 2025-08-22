import Meta from '../../../assets/icons/ai/meta.svg'
import Gemma from '../../../assets/icons/ai/gemma.svg'
import DeepSeek from '../../../assets/icons/ai/deepseek.svg'
import OpenAI from '../../../assets/icons/ai/openai.svg'
import Qwen from '../../../assets/icons/ai/qwen.svg'
import Mistral from '../../../assets/icons/ai/mistral.svg'
import Microsoft from '../../../assets/icons/ai/microsoft.svg'
import Ollama from '../../../assets/ollama.png';

export function getModelLogo(modelName: string) {
  const lowerCaseName = modelName.toLowerCase();

  if (lowerCaseName.includes('llama')) {
    return Meta;
  }
  if (lowerCaseName.includes('gemma')) {
    return Gemma;
  }
  if (lowerCaseName.includes('deepseek')) {
    return DeepSeek;
  }
  if (lowerCaseName.includes('gpt')) {
    return OpenAI;
  }
  if (lowerCaseName.includes('qwen')) {
    return Qwen;
  }
  if (lowerCaseName.includes('mistral') || lowerCaseName.includes('mixtral')) {
    return Mistral;
  }
  if (lowerCaseName.includes('phi')) {
    return Microsoft;
  }

  return Ollama;
}