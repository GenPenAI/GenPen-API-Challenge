--[[
  API Inspector

  No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

  The version of the OpenAPI document: 1.0.0
  
  Generated by: https://openapi-generator.tech
]]

-- user class
local user = {}
local user_mt = {
	__name = "user";
	__index = user;
}

local function cast_user(t)
	return setmetatable(t, user_mt)
end

local function new_user(id, username, first_name, last_name, email, password, phone, user_status)
	return cast_user({
		["id"] = id;
		["username"] = username;
		["firstName"] = first_name;
		["lastName"] = last_name;
		["email"] = email;
		["password"] = password;
		["phone"] = phone;
		["userStatus"] = user_status;
	})
end

return {
	cast = cast_user;
	new = new_user;
}
