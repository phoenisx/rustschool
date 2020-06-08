WIDTH = 1280
HEIGHT = 720

function love.load()
  love.window.setMode(WIDTH, HEIGHT, {
    fullscreen = false,
    resizable = false,
    vsync = true
  })
end

function love.draw()
  love.graphics.printf(
    'Hello Pong!',
    0,
    HEIGHT / 2 - 6, -- -6 is half height of default Text Height
    WIDTH,
    'center'
  )
end
