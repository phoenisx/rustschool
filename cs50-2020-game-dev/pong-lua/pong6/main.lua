--[[
  This chapter is about dividing entities into
  their own modules (Classes), so that they can
  be layout and rendered separately.

  Paddle - is a module that will control the left and right
  player paddles

  Ball - is a module that will control the speed, layout
  and redering of the Ball on screen

  main.lua - is responsible to manage players and their scores
]]

push = require('push')
Class = require 'class'

require('Paddle')
require('Ball')

WIDTH = 1280
HEIGHT = 720

VIRTUAL_WIDTH = 432
VIRTUAL_HEIGHT = 243

PADDLE_SPEED = 200
PADDLE_INIT_X = 10
PADDLE_INIT_Y = 30
PADDLE_WIDTH = 5
PADDLE_HEIGHT = 20

BALL_SIZE = 4

function love.load()
  love.graphics.setDefaultFilter('nearest', 'nearest')

  love.window.setTitle('Pong')
  math.randomseed(os.time())

  font = love.graphics.newFont('font.ttf', 8)
  scoreFont = love.graphics.newFont('font.ttf', 32)

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

  player1Score = 0
  player2Score = 0

  player1 = Paddle(PADDLE_INIT_X, PADDLE_INIT_Y, PADDLE_WIDTH, PADDLE_HEIGHT)
  player2 = Paddle(
    VIRTUAL_WIDTH - PADDLE_INIT_X,
    VIRTUAL_HEIGHT - PADDLE_INIT_Y,
    PADDLE_WIDTH,
    PADDLE_HEIGHT
  )

  ball = Ball(
    VIRTUAL_WIDTH / 2 - BALL_SIZE / 2,
    VIRTUAL_HEIGHT / 2 - BALL_SIZE / 2,
    BALL_SIZE,
    BALL_SIZE
  )

  gameState = 'start'
end

function love.keypressed(key)
    -- keys can be accessed by string name
    if key == 'escape' then
      -- function LÖVE gives us to terminate application
      love.event.quit()
    elseif key == 'enter' or key == 'return' then
      if gameState == 'start' then
        gameState = 'play'
      else
        gameState = 'start'
        ball:reset()
      end
    end
end

function love.update(delta)
  -- Player 1 Movement
  if love.keyboard.isDown('w') then
    -- paddle moves up, negative y axis
    player1.dy = -PADDLE_SPEED
  elseif love.keyboard.isDown('s') then
    -- paddle moves down, positive y axis
    player1.dy = PADDLE_SPEED
  else
    -- Don't change anything
    player1.dy = 0
  end

  -- Player 2 movement
  if love.keyboard.isDown('up') then
    player2.dy = -PADDLE_SPEED
  elseif love.keyboard.isDown('down') then
    player2.dy = PADDLE_SPEED
  else
    player2.dy = 0
  end

  if gameState == 'play' then
    ball:update(delta)
  end

  player1:update(delta)
  player2:update(delta)
end

function love.draw()
  -- begin rendering at virtual resolution
  push:apply('start')

  love.graphics.clear(40, 45, 52, 255)

  love.graphics.setFont(font)
  if gameState == 'start' then
    love.graphics.printf('Hello Start State!', 0, 20, VIRTUAL_WIDTH, 'center')
  else
    love.graphics.printf('Hello Play State!', 0, 20, VIRTUAL_WIDTH, 'center')
  end

  love.graphics.setFont(scoreFont)
  love.graphics.print(tostring(player1Score), VIRTUAL_WIDTH / 2 - 50,
    VIRTUAL_HEIGHT / 3)
  love.graphics.print(tostring(player2Score), VIRTUAL_WIDTH / 2 + 30,
    VIRTUAL_HEIGHT / 3)

  player1:render()
  player2:render()

  ball:render()

  displayFPS()

  push:apply('end')
end

function displayFPS()
  -- simple FPS display across all states
  love.graphics.setFont(font)
  love.graphics.setColor(0, 255, 0, 255)
  love.graphics.print('FPS: ' .. tostring(love.timer.getFPS()), 10, 10)
end
