import { invoke } from "@tauri-apps/api";

const headers = {
  'authority': 'www.iesdouyin.com',
  'sec-fetch-user': '?1',
  'upgrade-insecure-requests': '1',
  'User-Agent': 'Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1'
};

export async function request<T>(sec_uid: string, max_cursor?: string) {
  const query: QueryObject = {
    sec_uid,
    max_cursor,
    count: '15',
    reflow_source: 'reflow_page',
  };

  const params = serializeQuery(query);

  // 创建一个Request对象
  const request = {
    method: 'GET',
    url: `https://www.iesdouyin.com/web/api/v2/aweme/post?${params}`,
    headers,
    body: null,
  };
  // console.log(request);

  // 调用proxy_request函数
  const response: any = await invoke('proxy_request', { request });
  // 处理返回的响应
  const result = JSON.parse(response.body);
  console.log('Status:', response.status);
  console.log('Headers:', response.headers);
  console.log('Body:', response.body);
  console.log(result);
  return result;
}

interface QueryObject {
  [key: string]: string | undefined;
}

function serializeQuery(query: QueryObject): string {
  const params = new URLSearchParams();

  for (const key in query) {
    if (typeof query[key] === 'string') {
      params.append(key, <string>query[key]);
    }
  }

  return params.toString();
}

