--[[
  API Inspector

  No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

  The version of the OpenAPI document: 1.0.0
  
  Generated by: https://openapi-generator.tech
]]

-- error class
local error = {}
local error_mt = {
	__name = "error";
	__index = error;
}

local function cast_error(t)
	return setmetatable(t, error_mt)
end

local function new_error(code, message)
	return cast_error({
		["code"] = code;
		["message"] = message;
	})
end

return {
	cast = cast_error;
	new = new_error;
}
