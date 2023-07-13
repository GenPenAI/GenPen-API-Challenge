export JAVA_POST_PROCESS_FILE="/usr/local/bin/clang-format -i"
rm -rf ./dynamic-html
rm -rf ./mysql-schema
rm -rf ./jaxrs-spec
java -jar openapi-generator-cli.jar generate -g dynamic-html -o dynamic-html -i openapi.yaml
java -jar openapi-generator-cli.jar generate -g mysql-schema -o mysql-schema -i openapi.yaml --additional-properties=identifierNamingConvention=snake_case
java -jar openapi-generator-cli.jar generate -g jaxrs-spec --additional-properties=apiPackage=ai.genpen.api --additional-properties=invokerPackage=ai.genpen.api --additional-properties=modelPackage=ai.genpen.models -i openapi.yaml -o jaxrs-spec
rm -rf /home/xagau/Desktop/apache-tomcat-9.0.65/webapps/ROOT/api-inspector/api-docs
mkdir /home/xagau/Desktop/apache-tomcat-9.0.65/webapps/ROOT/api-inspector/api-docs
cp -rf ./dynamic-html/docs/* /home/xagau/Desktop/apache-tomcat-9.0.65/webapps/ROOT/api-inspector/api-docs/
cp -f ./openapi.yaml /home/xagau/Desktop/apache-tomcat-9.0.65/webapps/ROOT/api-inspector/openapi.yaml
/usr/bin/mvn -Dtomee-embedded-plugin.http=8080 package org.apache.tomee.maven:tomee-embedded-maven-plugin:7.0.5:run -f ./jaxrs-spec/pom.xml

