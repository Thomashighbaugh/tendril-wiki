const WIKI_LINK_REGEXP = new RegExp(
  /\[\[([a-zA-Z0-9\s?\-?'?:?_?’?(\|)?]+)\]\]/,
  "g"
);
const URL_REGEXP = new RegExp(
  /(?:(?:https?|ftp|file):\/\/|www\.|ftp\.)(?:\([-A-Z0-9+&@#\/%=~_|$?!:.]*\)|[-A-Z0-9+&@#\/%=~_|$?!:.])*(?:\([-A-Z0-9+&@#\/%=~_|$?!:.]*\)|[A-Z0-9+&@#\/%=~_|$])/,
  "ig"
);
const EMAIL_REGEXP = new RegExp(
  /[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*/,
  "igm"
);
const IMAGE_REGEXP = new RegExp(
  /.*\.(jpg|jpeg|png|gif|webp|apng|avif|jfif|pjpeg|pjp)$/,
  "i"
);

export function parseWikiLinks(text) {
  let finalString = text;
  for (const match of text.matchAll(WIKI_LINK_REGEXP)) {
    const alias = match[1].split("|");
    // handle aliased links: [[alias|actual page]]
    if (alias.length > 1) {
      finalString = finalString.replaceAll(
        match[0],
        `<a href="/${encodeURIComponent(alias[1])}">${alias[0]}</a>`
      );
    } else {
      finalString = finalString.replaceAll(
        match[0],
        `<a href="/${encodeURIComponent(match[1])}">${match[1]}</a>`
      );
    }
  }
  return finalString;
}

export function parseURLs(text) {
  for (const match of text.matchAll(URL_REGEXP)) {
    const [url, _] = match;
    if (isSpecialtyUrl(url)) {
      const specialParsed = parseSpecialtyUrl(url);
      text = text.replace(url, specialParsed);
    } else {
      text = text.replace(url, `<a href="${url}">${url}</a>`);
    }
  }
  return text;
}

export function parseEmails(text) {
  for (const match of text.matchAll(EMAIL_REGEXP)) {
    const [email, _] = match;
    text = text.replace(email, `<a href="mailto:${email}">${email}</a>`);
  }
  return text;
}

export function textToHtml(text) {
  return text
    .split("\n")
    .map((line) => parseEmails(parseURLs(parseWikiLinks(line))))
    .join("<br>");
}

export function htmlToText(el) {
  for (const anchor of el.querySelectorAll("a")) {
    if (anchor.href.includes("mailto:")) {
      anchor.replaceWith(anchor.innerText);
    } else {
      const path = decodeURIComponent(anchor.pathname).slice(1);
      const linkedPage = anchor.innerText;
      if (URL_REGEXP.test(linkedPage)) {
        anchor.replaceWith(linkedPage);
        return;
      }
      if (path === linkedPage) {
        anchor.replaceWith(`[[${linkedPage}]]`);
        return;
      } else {
        anchor.replaceWith(`[[${linkedPage}|${path}]]`);
      }
    }
  }
  for (const image of el.querySelectorAll("img")) {
    image.replaceWith(image.src);
  }
}

function isSpecialtyUrl(url) {
  return IMAGE_REGEXP.test(url);
}
function parseSpecialtyUrl(url) {
  if (IMAGE_REGEXP.test(url)) {
    return `<img src="${url}">`;
  }
  return "";
}
