// speedrun.com API wrapper

export const speedrunDotComApiRootUrl = '/https://www.speedrun.com/api/v1/';

export const cache = new window.Map();
export const get = async path => {
  if (!cache.has(path)) {
    const result = await doGet(path);
    cache.set(path, result);
    return result;
  } else {
    return cache.get(path);
  }
};

const doGet = async path => {
  const url = speedrunDotComApiRootUrl + path;
  const response = await window.fetch(url);
  const body = await response.json();
  if (body.status) {
    throw new Error(`${body.status}: ${body.message}`); 
  } else {
    return body.data;
  }
};
