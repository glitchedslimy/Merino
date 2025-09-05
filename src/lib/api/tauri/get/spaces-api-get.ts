import { invoke } from "@tauri-apps/api/core";
import type { Space } from "../interfaces/spaces-interface";

export async function getSpaces(): Promise<Space[]> {
    return invoke<Space[]>('get_spaces_cmd');
}