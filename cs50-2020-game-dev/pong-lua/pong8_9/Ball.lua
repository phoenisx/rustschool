Ball = Class{}

function resetVelocity()
  -- these variables are for keeping track of our velocity on both the
  -- X and Y axis, since the ball can move in two dimensions
  return {
    dx = 3 * math.random(-50, 50),
    dy = math.random(2) == 1 and -100 or 100
  }
end

function Ball:randomize(servingPlayer)
  ball.dy = math.random(-50, 50)
  if servingPlayer == 1 then
      ball.dx = math.random(140, 200)
  else
      ball.dx = -math.random(140, 200)
  end
end

function Ball:init(x, y, width, height)
  self.x = x
  self.y = y
  self.width = width
  self.height = height

  local deltaVelocity = resetVelocity()
  self.dy = deltaVelocity.dy
  self.dx = deltaVelocity.dx
end

function Ball:collides(paddle)
  if paddle.x > self.x + self.width or self.x > paddle.x + paddle.width then
    return false
  end

  if paddle.y > self.y + self.height or self.y > paddle.y + paddle.height then
    return false
  end

  return true
end


--[[
    Places the ball in the middle of the screen, with an initial random velocity
    on both axes.
]]
function Ball:reset()
  self.x = VIRTUAL_WIDTH / 2 - 2
  self.y = VIRTUAL_HEIGHT / 2 - 2
  local deltaVelocity = resetVelocity()
  self.dy = deltaVelocity.dy
  self.dx = deltaVelocity.dx
end

--[[
    Simply applies velocity to position, scaled by deltaTime.
]]
function Ball:update(dt)
  self.x = self.x + self.dx * dt
  self.y = self.y + self.dy * dt
end

function Ball:render()
  love.graphics.rectangle('fill', self.x, self.y, self.width, self.height)
end
