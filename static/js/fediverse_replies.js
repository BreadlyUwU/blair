console.log("🍞🐱 Fediverse Replies JS module loaded");
const bcrunContentRoot = document.querySelector("#content-box");

// Initialize once the i18n_current variable, the initTrans() function will determine if either the english or the french language is set. This var will be used to return more quickly the requested translated string.
let i18n_current;
// I should replace the hardcoded i18n array with an external json…
const i18n = [
  ["w-title", "Replies from the fediverse", "Réponses du fediverse"],
  [
    "w-link",
    "Link to the Mastodon post related to this article",
    "Lien vers le post Mastodon relié à cet article",
  ],
  [
    "howto",
    "To comment this article, reply to the post linked on the [Toot] button visible at the left of this window's titlebar with your Fediverse account (Mastodon, Akkoma, Misskey, Pixelfed, …).",
    "Pour commenter cet article, répondez au message lié au bouton [Toot] visible à gauche de la barre de titre de cette fenêtre avec votre compte Fediverse (Mastodon, Akkoma, Misskey, Pixelfed, …).",
  ],
  [
    "err1",
    "The Mastodon server is not currently available",
    "Le serveur Mastodon est actuellement indisponible",
  ],
  [
    "err404",
    "The fediverse post hasn't been found, a wrong link has probably been provided inside this article's metadata",
    "Le post du fediverse n'a pas été trouvé, un mauvais lien a probablement été fourni dans les metadonnées de cet article",
  ],
];
initTrans();

// ======================
// = YOOOOOOOOOOOOOOOOO =
// ======================
bcrBeginningOfTheEnd();

function bcrBeginningOfTheEnd() {
  // First it will check if the meta property "fediverse:rel-post" exists, and only create the comment box if it is the case.
  const hookedFediLink = hookFediLink();
  if (hookedFediLink != null) {
    console.log("Found:", hookedFediLink);
    console.log("Initializing the comment box");

    bcrunContentRoot.innerHTML += `
      <div class="window" id="fedi-replies">
        <div class="w-titlebar">
          <div class="w-left">
            <a target="_blank" href="${hookedFediLink}" title="${getTrans("w-link")}" alt="Toot!"><div class="w-btn fd-link"></div></a>
            <div class="w-title"><span>${getTrans("w-title")}</span></div>
          </div>
          <div class="w-right">
            <div class="w-btn minimize"></div>
            <div class="w-btn maximize"></div>
            <div class="w-btn close"></div>
          </div>
        </div>
        <div id="fr-howto"></div>
        <div id="fr-mainbox">
          <div id="fr-loading-bar"></div>
        </div>
      </div>
    `;

    writeHowTo();
    fetchReplies(hookedFediLink);
  }
}

// ==============
// = The Roster =
// ==============

// Captain Hook!
function hookFediLink() {
  // 1. The author write a link inside the frontmatter of a blog article under extra.fediverse_post.
  // 2. Zola will put the now existing page.extra.fediverse_post variable inside an HTML meta property named "fediverse:rel-post" (non-standard).
  // 3. Then, this JS program will *NOM* *NOM* *NOM* the property.
  let hookedLink = document.querySelector('[property="fediverse:rel-post"]');
  if (hookedLink != null) {
    return hookedLink.content;
  } else {
    console.log("No related fediverse post has been found.");
    return null;
  }
}

// Inuit Trans!
function initTrans() {
  const bcrunOgLocale = document.querySelector(
    '[property="og:locale"]',
  ).content;
  if (bcrunOgLocale != undefined && bcrunOgLocale == "fr_FR") {
    i18n_current = 2;
  } else {
    // Default to english
    i18n_current = 1;
  }
}

// Gay Trans!
function getTrans(search) {
  for (let item = 0, len = i18n.length; item < len; item++) {
    if (i18n[item][0] == search) {
      if (i18n_current != undefined) {
        return i18n[item][i18n_current];
      } else {
        // Fallback just in case
        return i18n[item][1];
      }
    }
  }
}

// Les courriers du cœur de Mister Flech!
function fetchReplies(url) {
  const fediUrl = new URL(url);
  const fediInstance = `${fediUrl.protocol}//${fediUrl.host}`;
  const fediPostID = fediUrl.href.split("/").pop();
  // I'll just assume that anything used with this script is a Mastodon server, because as everyone knows, Mastodon is the only Fediverse software running in the wild, nothing else exists.
  const apiEndpoint = `/api/v1/statuses/`;

  fetch(`${fediInstance}${apiEndpoint}${fediPostID}/context`)
    .then((reponse) => {
      // The program will stop here if not receiving a code 200 from the server
      if (!reponse.ok) {
        throw new Error(`Error: ${reponse.status}`);
      }
      // Reseting the #fr-mainbox to remove the loading bar
      document.querySelector("#fr-mainbox").innerHTML = ``;
      console.log("Message(s) retrieved");
      return reponse.json();
    })
    .then((data) => {
      data.descendants.forEach((element) => {
        console.log(`Parsing message id:${element.id}`);

        let userAccount = element.account.acct.split("@"); // [0] = username; [1] = server
        // if the account's server is "undefinied", it's because it's a local account.
        if (userAccount[1] == undefined) {
          userAccount[1] = `${fediUrl.hostname}`;
        }
        let postDate = element.created_at.split("T");
        let postTime = postDate[1].split(".");
        let postBody = element.content;

        // Parsing the account-linked emojos and replacing the existing :shortcode: from the user's display name
        let userDisplayName = element.account.display_name;
        element.account.emojis.forEach((emojo) => {
          let emojoShortcode = `:${emojo.shortcode}:`;
          userDisplayName = userDisplayName.replace(
            emojoShortcode,
            `<img class="emojo" src="${emojo.url}" title="${emojoShortcode}" alt="${emojoShortcode}">`,
          );
        });

        // Parsing the emojos *from the content* and replacing the existing :shortcode: inside the body of the message with the right img
        element.emojis.forEach((emojo) => {
          let emojoShortcode = `:${emojo.shortcode}:`;
          postBody = postBody.replace(
            emojoShortcode,
            `<img class="emojo" src="${emojo.url}" title="${emojoShortcode}" alt="${emojoShortcode}">`,
          );
        });

        // Parsing the image attachments (if any) and adding them to the body
        element.media_attachments.forEach((attachment) => {
          postBody += `<a target="_blank" href="${attachment.url}"><img class="attachment" src="${attachment.preview_url}" alt="${attachment.description}"></a>`;
        });

        document.querySelector("#fr-mainbox").innerHTML += `
          <div id="${element.id}" class="reply-box">
            <div class="left-box">
              <div class="avatar-box">
                <a target="_blank" href="${element.account.avatar}"><img class="avatar" src="${element.account.avatar}" width=100%></a>
              </div>
              <div class="name-box">
                <a target="_blank" href="${element.account.url}"><span class="displayname">${userDisplayName}</span></a>
              </div>
            </div>
            <div class="right-box">
              <div class="infobar">
                <a target="_blank" href="${element.account.avatar}"><img class="avatar" src="${element.account.avatar}"></a>
                <a target="_blank" href="${element.url}"><span class="date">${postDate[0]}</span><span class="hour">${postTime[0]}</span></a>
                <span class="server">// ${userAccount[0]}@${userAccount[1]}</span>
              </div>
              <div class="message-body">
                ${postBody}
              </div>
            </div>
          </div>
        `;
      });
    })
    .catch((error) => {
      document.querySelector("#fr-mainbox").innerHTML = `
        <h3>An error occured:</h3>
        <p>${error}</p>
      `;
    });
}

// White Talk-Talk!
function writeHowTo() {
  document.querySelector("#fr-howto").innerHTML += `
    <center><span><b>${getTrans("howto")}
  `;
}

// Dummy Scrappy-Doo!
// (Will be removed in the future, as it is currently useless… like Scrappy-Doo…)
function dummyReply() {
  document.querySelector("#fr-mainbox").innerHTML += `
    <div id="dummy" class="reply-box">
      <div class="left-box">
        <center>
          <img class="avatar" src="https://mstdn.breadcat.run/system/accounts/avatars/114/909/081/373/398/620/original/d9e32ce1be93a5c6.png" width=100%><br>
          <span class="username">Dummy</span><br>
          <span class="server">breadcat.run</span>
        </center>
      </div>
      <div class="right-box">
        <p>Yo!</p>
        <p>Yo! Yo! Yo! Yo!</p>
        <p>Hey, <a href="#">#Yo</a></p>
      </div>
    </div>
  `;
}
