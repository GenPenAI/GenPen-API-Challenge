with Ada.IO_Exceptions;
with AWS.Config.Set;
with Swagger.Servers.AWS;
with Swagger.Servers.Applications;
with Util.Strings;
with Util.Log.Loggers;
with Util.Properties;
with Util.Properties.Basic;
with .Servers;
procedure .Server is
   procedure Configure (Config : in out AWS.Config.Object);

   use Util.Properties.Basic;

   CONFIG_PATH  : constant String := ".properties";
   Port : Natural := 8080;

   procedure Configure (Config : in out AWS.Config.Object) is
   begin
      AWS.Config.Set.Server_Port (Config, Port);
      AWS.Config.Set.Max_Connection (Config, 8);
      AWS.Config.Set.Accept_Queue_Size (Config, 512);
   end Configure;

   App     : aliased Swagger.Servers.Applications.Application_Type;
   WS      : Swagger.Servers.AWS.AWS_Container;
   Log     : constant Util.Log.Loggers.Logger := Util.Log.Loggers.Create (".Server");
   Props   : Util.Properties.Manager;
begin
   Props.Load_Properties (CONFIG_PATH);
   Util.Log.Loggers.Initialize (Props);

   Port := Integer_Property.Get (Props, "swagger.port", Port);
   App.Configure (Props);
   .Servers.Server_Impl.Register (App);

   WS.Configure (Configure'Access);
   WS.Register_Application ("/openapi_jaxrs_server_1.0.0/api/v3", App'Unchecked_Access);
   App.Dump_Routes (Util.Log.INFO_LEVEL);
   Log.Info ("Connect your browser to: http://localhost:{0}/openapi_jaxrs_server_1.0.0/api/v3/ui/index.html",
             Util.Strings.Image (Port));

   WS.Start;

   delay 6000.0;

exception
   when Ada.IO_Exceptions.Name_Error =>
      Log.Error ("Cannot read application configuration file {0}", CONFIG_PATH);
end .Server;
