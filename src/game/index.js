import * as PIXI from 'pixi.js';

// The application will create a renderer using WebGL, if possible,
// with a fallback to a canvas render. It will also setup the ticker
// and the root stage PIXI.Container
// https://pixijs.download/release/docs/PIXI.Application.html
const app = new PIXI.Application({
  antialias: true,
  view: document.getElementById('game-canvas')
});

/*
window.addEventListener("resize", function() {
  app.renderer.resize(window.innerWidth, window.innerHeight);
});
*/

// load the texture we need
/*
app.loader.add('cat', 'cat.png').load((loader, resources) => {
  // This creates a texture from a 'cat.png' image
  const cat = new PIXI.Sprite(resources.cat.texture);

  // Setup the position of the cat
  cat.x = app.renderer.width / 2;
  cat.y = app.renderer.height / 2;

  // Rotate around the center
  cat.anchor.x = 0.5;
  cat.anchor.y = 0.5;

  // Add the cat to the scene we are building
  app.stage.addChild(cat);

  // Listen for frame updates
  app.ticker.add(() => {
     // each frame we spin the cat around a bit
    cat.rotation += 0.01;
  });
});
*/


const loaderBarWidth = app.stage.width;

//Create the loader bar
const loaderBar = new PIXI.Container();
loaderBar.position.set(0, 200);
app.stage.addChild(loaderBar);

//Create the black background rectangle
let innerBar = new PIXI.Graphics();
innerBar.beginFill(0xccee33);
innerBar.drawRect(0, 0, loaderBarWidth, 100);
innerBar.endFill();
loaderBar.addChild(innerBar);

//Create the front red rectangle
let outerBar = new PIXI.Graphics();
outerBar.beginFill(0xFF3300);
outerBar.drawRect(0, 0, loaderBarWidth, 100);
outerBar.endFill();
outerBar.scale.x = 0;
loaderBar.addChild(outerBar);
loaderBar.outer = outerBar;

let style = new PIXI.TextStyle({
  fontFamily: "Futura",
  fontSize: 64,
  fill: "white"
});
const message = new PIXI.Text("Welcommen", style);
message.x = 120;
message.y = app.stage.height;
app.stage.addChild(message);

// https://pixijs.download/release/docs/PIXI.Loader.html
const loader = PIXI.Loader.shared;
loader.add([
  '/layer_01_1920x1080.png',
  '/layer_02_1920x1080.png',
  '/layer_03_1920x1080.png',
  '/layer_04_1920x1080.png',
  '/layer_05_1920x1080.png',
  '/layer_06_1920x1080.png',
  '/layer_07_1920x1080.png',
  '/layer_08_1920x1080.png',
]);
loader.onProgress.add((loader, resource) => {

  //Display the file `url` currently being loaded
  console.log("loading: " + resource.url);

  //Display the percentage of files currently loaded
  console.log("progress: " + loader.progress + "%");
  outerBar.scale.x = loader.progress / 100;

  //If you gave your files names as the first argument
  //of the `add` method, you can access them like this
  //console.log("loading: " + resource.name);
});

function keyboard(value) {
  let key = {};
  key.value = value;
  key.isDown = false;
  key.isUp = true;
  key.press = undefined;
  key.release = undefined;
  //The `downHandler`
  key.downHandler = event => {
    if (event.key === key.value) {
      if (key.isUp && key.press) key.press();
      key.isDown = true;
      key.isUp = false;
      event.preventDefault();
    }
  };

  //The `upHandler`
  key.upHandler = event => {
    if (event.key === key.value) {
      if (key.isDown && key.release) key.release();
      key.isDown = false;
      key.isUp = true;
      event.preventDefault();
    }
  };

  //Attach event listeners
  const downListener = key.downHandler.bind(key);
  const upListener = key.upHandler.bind(key);

  window.addEventListener(
    "keydown", downListener, false
  );
  window.addEventListener(
    "keyup", upListener, false
  );

  // Detach event listeners
  key.unsubscribe = () => {
    window.removeEventListener("keydown", downListener);
    window.removeEventListener("keyup", upListener);
  };

  return key;
}

//Capture the keyboard arrow keys
let left = keyboard("ArrowLeft"),
  up = keyboard("ArrowUp"),
  right = keyboard("ArrowRight"),
  down = keyboard("ArrowDown");


let parallax01, parallax02, parallax03, parallax04, parallax05, parallax06, parallax07, parallax08;

const moveLeft = () => {
  parallax08.tilePosition.x += 0.1;
  parallax07.tilePosition.x += 0.2;
  parallax06.tilePosition.x += 0.3;
  parallax05.tilePosition.x += 0.4;
  parallax04.tilePosition.x += 0.5;
  parallax03.tilePosition.x += 0.6;
  parallax02.tilePosition.x += 0.7;
  parallax01.tilePosition.x += 0.8;
};

const moveRight = () => {
  parallax08.tilePosition.x -= 0.1;
  parallax07.tilePosition.x -= 0.2;
  parallax06.tilePosition.x -= 0.3;
  parallax05.tilePosition.x -= 0.4;
  parallax04.tilePosition.x -= 0.5;
  parallax03.tilePosition.x -= 0.6;
  parallax02.tilePosition.x -= 0.7;
  parallax01.tilePosition.x -= 0.8;
};


left.press = () => {
  moveLeft();
};
right.press = () => {
  moveRight();
};


loader.load((loader, resources) => {
  console.log("All files loaded");

  // https://pixijs.download/release/docs/PIXI.TilingSprite.html
  parallax01 = new PIXI.TilingSprite(resources['/layer_01_1920x1080.png'].texture, 1920, 1080);
  parallax02 = new PIXI.TilingSprite(resources['/layer_02_1920x1080.png'].texture, 1920, 1080);
  parallax03 = new PIXI.TilingSprite(resources['/layer_03_1920x1080.png'].texture, 1920, 1080);
  parallax04 = new PIXI.TilingSprite(resources['/layer_04_1920x1080.png'].texture, 1920, 1080);
  parallax05 = new PIXI.TilingSprite(resources['/layer_05_1920x1080.png'].texture, 1920, 1080);
  parallax06 = new PIXI.TilingSprite(resources['/layer_06_1920x1080.png'].texture, 1920, 1080);
  parallax07 = new PIXI.TilingSprite(resources['/layer_07_1920x1080.png'].texture, 1920, 1080);
  parallax08 = new PIXI.TilingSprite(resources['/layer_08_1920x1080.png'].texture, 1920, 1080);

  parallax01.position.set(0, -100);
  parallax02.position.set(0, -100);
  parallax03.position.set(0, -100);
  parallax04.position.set(0, -100);
  parallax05.position.set(0, -100);
  parallax06.position.set(0, -100);
  parallax07.position.set(0, -100);
  parallax08.position.set(0, -100);

  parallax01.tilePosition.x = 0;
  parallax01.tilePosition.y = 0;

  parallax02.tilePosition.x = 0;
  parallax02.tilePosition.y = 0;

  app.stage.addChild(parallax08);
  app.stage.addChild(parallax07);
  app.stage.addChild(parallax06);
  app.stage.addChild(parallax05);
  app.stage.addChild(parallax04);
  app.stage.addChild(parallax03);
  app.stage.addChild(parallax02);
  app.stage.addChild(parallax01);

  app.ticker.add(delta => gameLoop(delta));
});

function gameLoop(delta) {
  if (left.isDown) {
    moveLeft();
  }
  if (right.isDown) {
    moveRight();
  }
}
