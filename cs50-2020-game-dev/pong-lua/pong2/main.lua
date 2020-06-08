push = require('push')

WIDTH = 1280
HEIGHT = 720

VIRTUAL_WIDTH = 432
VIRTUAL_HEIGHT = 243

function love.load()
  love.graphics.setDefaultFilter('nearest', 'nearest')

  font = love.graphics.newFont('font.ttf', 8)

  love.graphics.setFont(font)

  push:setupScreen(
    VIRTUAL_WIDTH,
    VIRTUAL_HEIGHT,
    WIDTH,
    HEIGHT,
    {
      fullscreen = false,
      resizable = false,
      vsync = true
    }
  )
end

function love.keypressed(key)
    -- keys can be accessed by string name
    if key == 'escape' then
        -- function LÖVE gives us to terminate application
        love.event.quit()
    end
end

function love.draw()
  -- begin rendering at virtual resolution
  push:apply('start')

  love.graphics.clear(40, 45, 52, 255)

  love.graphics.printf(
    'Hello Pong!',
    0,
    20,
    VIRTUAL_WIDTH,
    'center'
  )

  love.graphics.rectangle('fill', 10, 30, 5, 20)
  love.graphics.rectangle(
    'fill',
    VIRTUAL_WIDTH - 10,
    -- It's -50, because we want to place it to the right
    -- having a padding of 30points, where 20 extra points
    -- is due tot he rectangle's width size.
    VIRTUAL_HEIGHT - 50,
    5,
    20
  )

  push:apply('end')
end
