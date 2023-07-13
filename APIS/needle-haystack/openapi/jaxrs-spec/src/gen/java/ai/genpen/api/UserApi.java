package ai.genpen.api;

import java.util.Date;
import java.util.List;
import ai.genpen.models.User;

import javax.ws.rs.*;
import javax.ws.rs.core.Response;

import io.swagger.annotations.*;

import java.io.InputStream;
import java.util.Map;
import java.util.List;
import javax.validation.constraints.*;
import javax.validation.Valid;

@Path("/user")
@Api(description = "the user API")
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaJAXRSSpecServerCodegen", date = "2023-07-13T07:53:52.166177300-08:00[GMT-08:00]")
public class UserApi {

    @POST
    @Consumes({ "application/json", "application/xml", "application/x-www-form-urlencoded" })
    @Produces({ "application/json", "application/xml" })
    @ApiOperation(value = "Create user", notes = "This can only be done by the logged in user.", response = User.class, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = User.class)
    })
    public Response createUser(@Valid User user) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/createWithList")
    @Consumes({ "application/json" })
    @Produces({ "application/xml", "application/json" })
    @ApiOperation(value = "Creates list of users with given input array", notes = "Creates list of users with given input array", response = User.class, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Successful operation", response = User.class),
        @ApiResponse(code = 200, message = "successful operation", response = Void.class)
    })
    public Response createUsersWithListInput(@Valid List<User> user) {
        return Response.ok().entity("magic!").build();
    }

    @DELETE
    @Path("/{username}")
    @ApiOperation(value = "Delete user", notes = "This can only be done by the logged in user.", response = Void.class, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 400, message = "Invalid username supplied", response = Void.class),
        @ApiResponse(code = 404, message = "User not found", response = Void.class)
    })
    public Response deleteUser(@PathParam("username") @ApiParam("The name that needs to be deleted") String username) {
        return Response.ok().entity("magic!").build();
    }

    @GET
    @Path("/{username}")
    @Produces({ "application/xml", "application/json" })
    @ApiOperation(value = "Get user by user name", notes = "", response = User.class, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = User.class),
        @ApiResponse(code = 400, message = "Invalid username supplied", response = Void.class),
        @ApiResponse(code = 404, message = "User not found", response = Void.class)
    })
    public Response getUserByName(@PathParam("username") @ApiParam("The name that needs to be fetched. Use user1 for testing. ") String username) {
        return Response.ok().entity("magic!").build();
    }

    @GET
    @Path("/login")
    @Produces({ "application/xml", "application/json" })
    @ApiOperation(value = "Logs user into the system", notes = "", response = String.class, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = String.class),
        @ApiResponse(code = 400, message = "Invalid username/password supplied", response = Void.class)
    })
    public Response loginUser(@QueryParam("username")  @ApiParam("The user name for login")  String username,@QueryParam("password")  @ApiParam("The password for login in clear text")  String password) {
        return Response.ok().entity("magic!").build();
    }

    @GET
    @Path("/logout")
    @ApiOperation(value = "Logs out current logged in user session", notes = "", response = Void.class, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class)
    })
    public Response logoutUser() {
        return Response.ok().entity("magic!").build();
    }

    @PUT
    @Path("/{username}")
    @Consumes({ "application/json", "application/xml", "application/x-www-form-urlencoded" })
    @ApiOperation(value = "Update user", notes = "This can only be done by the logged in user.", response = Void.class, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class)
    })
    public Response updateUser(@PathParam("username") @ApiParam("name that need to be deleted") String username,@Valid User user) {
        return Response.ok().entity("magic!").build();
    }
}
