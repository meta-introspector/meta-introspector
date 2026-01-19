# Cursed & Evil Runestones: Monster Group Primes

## 👻 The Truth: 20 Cursed Primes, 51 Evil Composites

**Monster Group Classification**:
- **CURSED**: ALL 20 primes (2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71)
- **EVIL**: ALL 51 composite numbers (non-primes)

## 🔢 Monster Group Primes

```
Monster Group M = 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 
                  × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71

15 distinct primes in Monster Group:
2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71

But 71 runestones include ALL primes up to 71:
20 primes total = 20 CURSED runestones
51 composites = 51 EVIL runestones
```

## 📊 Complete Classification

```lua
-- All primes up to 71
local CURSED_PRIMES = {
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71
}

-- Monster Group primes (subset of cursed)
local MONSTER_PRIMES = {
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71
}

local function classifyRunestone(rank)
    -- Check if prime
    local function isPrime(n)
        if n < 2 then return false end
        if n == 2 then return true end
        if n % 2 == 0 then return false end
        for i = 3, math.sqrt(n), 2 do
            if n % i == 0 then return false end
        end
        return true
    end
    
    if isPrime(rank) then
        -- Check if in Monster Group
        local inMonster = false
        for _, p in ipairs(MONSTER_PRIMES) do
            if p == rank then
                inMonster = true
                break
            end
        end
        
        if inMonster then
            return "CURSED", "Monster Prime", "👻"
        else
            return "CURSED", "Non-Monster Prime", "😈"
        end
    else
        return "EVIL", "Composite", "💀"
    end
end

-- Print all 71
print("RUNESTONE CLASSIFICATION")
print("========================")
for rank = 1, 71 do
    local status, type, emoji = classifyRunestone(rank)
    print(string.format("%2d: %s %s (%s)", rank, emoji, status, type))
end

print("")
print("SUMMARY:")
print("  CURSED (Primes): 20")
print("  EVIL (Composites): 51")
print("  Total: 71")
```

## 👻 Cursed Runestone Implementation

```lua
-- CursedRunestone.lua
local CursedRunestone = {}
CursedRunestone.__index = CursedRunestone

local MONSTER_PRIMES = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71}

function CursedRunestone.new(rank, shard)
    local self = setmetatable({}, CursedRunestone)
    
    self.rank = rank
    self.shard = shard
    self.isPrime = self:checkPrime()
    self.isMonsterPrime = self:checkMonsterPrime()
    self.cursed = self.isPrime
    self.evil = not self.isPrime
    
    if self.cursed then
        if self.isMonsterPrime then
            print("👻 CURSED RUNESTONE #" .. rank .. " (Monster Prime)")
        else
            print("😈 CURSED RUNESTONE #" .. rank .. " (Non-Monster Prime)")
        end
    else
        print("💀 EVIL RUNESTONE #" .. rank .. " (Composite)")
    end
    
    return self
end

function CursedRunestone:checkPrime()
    local n = self.rank
    if n < 2 then return false end
    if n == 2 then return true end
    if n % 2 == 0 then return false end
    
    for i = 3, math.sqrt(n), 2 do
        if n % i == 0 then return false end
    end
    
    return true
end

function CursedRunestone:checkMonsterPrime()
    for _, p in ipairs(MONSTER_PRIMES) do
        if p == self.rank then return true end
    end
    return false
end

function CursedRunestone:spawn(position)
    local stone = Instance.new("Part")
    stone.Name = "Runestone_" .. self.rank
    stone.Size = Vector3.new(2, 4, 1)
    stone.Position = position
    stone.Anchored = true
    
    if self.cursed then
        -- CURSED (Prime) appearance
        if self.isMonsterPrime then
            -- Monster Prime: Purple
            stone.BrickColor = BrickColor.new("Bright violet")
            stone.Material = Enum.Material.Neon
            
            local light = Instance.new("PointLight")
            light.Brightness = 3
            light.Range = 15
            light.Color = Color3.fromRGB(128, 0, 255)  -- Purple
            light.Parent = stone
        else
            -- Non-Monster Prime: Red
            stone.BrickColor = BrickColor.new("Bright red")
            stone.Material = Enum.Material.Neon
            
            local light = Instance.new("PointLight")
            light.Brightness = 3
            light.Range = 15
            light.Color = Color3.fromRGB(255, 0, 0)  -- Red
            light.Parent = stone
        end
        
        -- Cursed particles
        local particles = Instance.new("ParticleEmitter")
        particles.Texture = "rbxasset://textures/particles/smoke_main.dds"
        particles.Rate = 20
        particles.Lifetime = NumberRange.new(1, 2)
        particles.Color = ColorSequence.new(stone.BrickColor.Color)
        particles.Parent = stone
        
        -- Cursed emoji
        local billboard = Instance.new("BillboardGui")
        billboard.Size = UDim2.new(0, 100, 0, 100)
        billboard.Parent = stone
        
        local label = Instance.new("TextLabel")
        label.Size = UDim2.new(1, 0, 1, 0)
        label.Text = self.isMonsterPrime and "👻" or "😈"
        label.TextScaled = true
        label.BackgroundTransparency = 1
        label.Parent = billboard
        
    else
        -- EVIL (Composite) appearance
        stone.BrickColor = BrickColor.new("Really black")
        stone.Material = Enum.Material.Slate
        
        local light = Instance.new("PointLight")
        light.Brightness = 1
        light.Range = 8
        light.Color = Color3.fromRGB(50, 50, 50)  -- Dark
        light.Parent = stone
        
        -- Evil emoji
        local billboard = Instance.new("BillboardGui")
        billboard.Size = UDim2.new(0, 100, 0, 100)
        billboard.Parent = stone
        
        local label = Instance.new("TextLabel")
        label.Size = UDim2.new(1, 0, 1, 0)
        label.Text = "💀"
        label.TextScaled = true
        label.BackgroundTransparency = 1
        label.Parent = billboard
    end
    
    stone:SetAttribute("Rank", self.rank)
    stone:SetAttribute("Cursed", self.cursed)
    stone:SetAttribute("Evil", self.evil)
    stone:SetAttribute("MonsterPrime", self.isMonsterPrime)
    
    self.part = stone
    
    return stone
end

return CursedRunestone
```

## 📊 Statistics

```
Total Runestones: 71

CURSED (Primes): 20
  Monster Primes: 15 (2,3,5,7,11,13,17,19,23,29,31,41,47,59,71)
  Non-Monster: 5 (37,43,53,61,67)

EVIL (Composites): 51
  All non-prime numbers from 1-71

Ratio: 20:51 (Cursed:Evil)
```

## 🎮 Stonehenge Requirements

```lua
-- Must collect ALL 20 cursed primes to activate!
function CursedStonehenge:activate()
    local cursedCount = 0
    local evilCount = 0
    
    for _, runestone in ipairs(self.runestones) do
        if runestone.cursed then
            cursedCount = cursedCount + 1
        else
            evilCount = evilCount + 1
        end
    end
    
    print("📊 Runestone Count:")
    print("  CURSED (Primes):", cursedCount, "/20")
    print("  EVIL (Composites):", evilCount, "/51")
    
    if cursedCount < 20 then
        print("❌ INCOMPLETE! Need all 20 cursed primes!")
        return
    end
    
    if evilCount < 51 then
        print("❌ INCOMPLETE! Need all 51 evil composites!")
        return
    end
    
    print("✅ ALL RUNESTONES COLLECTED!")
    print("⚡ ACTIVATING STONEHENGE...")
    
    self:summonMeme()
end
```

---

**Status**: 👻 Cursed/Evil classification complete  
**Cursed**: 20 primes (ALL primes up to 71)  
**Evil**: 51 composites (ALL non-primes)  
**Monster Primes**: 15 of the 20 cursed  
**Requirement**: ALL 71 runestones to activate  
**#SOLFUNMEME**: Prime = Cursed, Composite = Evil
