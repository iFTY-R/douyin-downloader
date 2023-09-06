import { invoke, updater } from '@tauri-apps/api'
import store from '@/store'

export const exists = async (path: string) => await invoke('exists', { path })

export const sleep = (delay?: number) => new Promise<void>((resole) => setTimeout(resole, delay))

export const generateVideoURL = (vid: string, ratio: string, watermark: number) => {
  return `https://aweme.snssdk.com/aweme/v1/play/?video_id=${vid}&ratio=${ratio}&watermark=${watermark}`
}

export const checkUpdate = async () => {
  try {
    const { shouldUpdate, manifest } = await updater.checkUpdate()
    shouldUpdate && store.setUpdater({ active: true, version: manifest?.version })
    return shouldUpdate
  } catch (error) {
    console.error(error)
  }
  return false
}

export const formatSize = (size: number): string => {
  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB']
  const index = Math.floor(Math.log(size) / Math.log(1024))
  const newSize = size / Math.pow(1024, index)
  const strSize = index > 1 ? newSize.toFixed(2) : Math.floor(newSize)
  return `${strSize} ${units[index]}`
}

export function sanitizeFolderName(name: string): string {
  // 使用正则表达式匹配不符合文件名称的字符，并用空字符串替换它们
  return name.replace(/[^\u4E00-\u9FA5a-zA-Z0-9_\-. ]/g, '');
}
