--[[
  API Inspector

  No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

  The version of the OpenAPI document: 1.0.0
  
  Generated by: https://openapi-generator.tech
]]

-- ddd class
local ddd = {}
local ddd_mt = {
	__name = "ddd";
	__index = ddd;
}

local function cast_ddd(t)
	return setmetatable(t, ddd_mt)
end

local function new_ddd(name)
	return cast_ddd({
		["name"] = name;
	})
end

return {
	cast = cast_ddd;
	new = new_ddd;
}