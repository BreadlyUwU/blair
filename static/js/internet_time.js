// Little script with just one purpose: Showing the "Internet Time" at the top of the website

// getSwatchTime function source: https://github.com/swatchtime/sample-code/blob/main/js/getSwatchTime.js
function getSwatchTime(date = new Date()) {
  // seconds since UTC midnight
  const utcSeconds = date.getUTCHours() * 3600 + date.getUTCMinutes() * 60 + date.getUTCSeconds();
  // Biel (UTC+1) seconds since midnight
  const bielSeconds = (utcSeconds + 3600 + 24 * 3600) % (24 * 3600);
  const beats = String(Math.floor(bielSeconds / 86.4) % 1000).padStart(3, '0');
  return `@${beats}`;
}

function drawSwatchTime() {
    const beats = getSwatchTime();
    const targetDiv = document.querySelector("#internet-time");
    targetDiv.textContent = beats;
    console.log("Internet time (.beat) refreshed:", beats);
}

drawSwatchTime()
setInterval(drawSwatchTime, 86400) // <- Refreshing every 1min & 26,4sec