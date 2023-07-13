export JAVA_POST_PROCESS_FILE="/usr/local/bin/clang-format -i"
rm -rf /home/xagau/Desktop/openapi/dynamic-html
rm -rf /home/xagau/Desktop/openapi/mysql-schema
rm -rf /home/xagau/Desktop/openapi/kotlin
java -jar openapi-generator-cli.jar generate -g dynamic-html -o dynamic-html -i openapi.yaml
java -jar openapi-generator-cli.jar generate -g mysql-schema -o mysql-schema -i openapi.yaml --additional-properties=identifierNamingConvention=snake_case
java -jar openapi-generator-cli.jar generate -g kotlin -i openapi.yaml -o kotlin
rm -rf /home/xagau/Desktop/apache-tomcat-9.0.65/webapps/ROOT/api-inspector/api-docs
mkdir /home/xagau/Desktop/apache-tomcat-9.0.65/webapps/ROOT/api-inspector/api-docs
cp -rf /home/xagau/Desktop/openapi/dynamic-html/docs/* /home/xagau/Desktop/apache-tomcat-9.0.65/webapps/ROOT/api-inspector/api-docs/
cp /home/xagau/Desktop/openapi/openapi.yaml /home/xagau/Desktop/apache-tomcat-9.0.65/webapps/ROOT/api-inspector/openapi.yaml


