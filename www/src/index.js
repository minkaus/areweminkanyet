import * as aplang from "areweaplangyet-wasm";

const REPO = "minkan-chat/server";
const START_TIME = 1626110100000.0;

function getPhrase(days) {
  if (days == 0) {
    return "A good day for privacy-respecting messaging";
  } else {
    return "Minkaus. Get off real life";
  }
}

async function renderProgress() {
  const res = await aplang.fetch_latest_commit(REPO, START_TIME);

  const commit = `https://github.com/${REPO}/commit/${res.sha}`;
  const phrase = getPhrase(res.days);

  const fmt = `${res.daysTotal} days without Minkan, ${res.days} days since the last \
    <a href="${commit}">commit</a>. ${phrase}!`;
  document.getElementById("aplang-info").innerHTML = fmt;
}

renderProgress();
