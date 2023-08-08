import * as aplang from "areweaplangyet-wasm";

const REPO = "APLanguage/aplang-rs";
const START_TIME = 1613750769000.0;

function getPhrase(days) {
  if (days == 0) {
    return "A good day to be AP";
  } else {
    return "Get off Discord";
  }
}

async function renderProgress() {
  const res = await aplang.fetch_latest_commit(REPO, START_TIME);

  const commit = `https://github.com/${REPO}/commit/${res.sha}`;
  const phrase = getPhrase(res.days);

  const fmt = `${res.daysTotal} days without APLang, ${res.days} days since the last \
    <a href="${commit}">commit</a>. ${phrase}!`;
  document.getElementById("aplang-info").innerHTML = fmt;
}

renderProgress();
