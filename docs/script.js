import init, { vis } from "./../wasm_program/pkg/wasm_program.js";

let input = "";
let output = "";
let tBar;
let turn;
let speed;
let playButton;
let prev;

async function initialize() {
  await init();

  tBar = document.getElementById("t_bar");
  turn = document.getElementById("turn");
  speed = document.getElementById("speed");
  playButton = document.getElementById("play");

  playButton.onclick = togglePlay;

  await updateInput();
  updateOutput();
  autoplay();
}

async function updateInput() {
  const fileName = document.getElementById("input").value;
  try {
    const response = await fetch(`input/${fileName}.txt`);
    input = await response.text();
  } catch (error) {
    console.error("Error loading input file:", error);
  }
}
window.updateInput = updateInput;

function updateOutput() {
  output = document.getElementById("output").value.replace(/[\r\n]+/g, "");
  document.getElementById("output").value = output;
  tBar.max = output.length;
  tBar.value = output.length;
  turn.max = output.length;
  turn.value = output.length;
  visualize();
}
window.updateOutput = updateOutput;

function visualize() {
  try {
    const ret = vis(input, output.slice(0, tBar.value));
    document.getElementById("result").innerHTML = ret.vis;
  } catch (error) {
    console.error("Visualization error:", error);
    document.getElementById("result").innerHTML = "<p>Invalid</p>";
  }
}

function updateTurn(turnValue) {
  const maxTurn = Number(turn.max);
  const clampedTurn = Math.min(Math.max(0, turnValue), maxTurn);
  tBar.value = clampedTurn;
  turn.value = clampedTurn;
  visualize();
}
window.updateTurn = updateTurn;

function startAutoplay() {
  if (Number(turn.value) >= Number(tBar.max)) {
    turn.value = 0;
  }
  prev = Date.now();
  playButton.value = "■";
  updateTurn(Number(turn.value));
}

function togglePlay() {
  playButton.value === "■" ? (playButton.value = "▶") : startAutoplay();
}
window.togglePlay = togglePlay;

function autoplay() {
  if (playButton.value === "■") {
    const now = Date.now();
    const interval = 1000;
    const elapsed = (now - prev) * speed.value;
    if (elapsed >= interval) {
      const increment = Math.floor(elapsed / interval);
      prev += Math.floor((increment * interval) / speed.value);
      updateTurn(Number(turn.value) + increment);
      if (Number(turn.value) >= Number(tBar.max)) {
        playButton.value = "▶";
      }
    }
  }
  requestAnimationFrame(autoplay);
}

initialize();
