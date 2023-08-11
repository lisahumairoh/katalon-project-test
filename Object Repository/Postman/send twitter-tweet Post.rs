<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>send twitter-tweet Post</name>
   <tag></tag>
   <elementGuidId>bc49ee29-c963-4428-9bdb-607418e8abc3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;ConnectionId\&quot;: 576,\n\t\&quot;Subject\&quot;: \&quot;Hallo Semua :)\&quot;,\n\t\&quot;MethodId\&quot;: \&quot;POST\&quot;,\n\t\&quot;From\&quot;: \&quot;738567483435556864\&quot;,\n\t\&quot;To\&quot;: \&quot;738567483435556864\&quot;,\n\t\&quot;Attachments\&quot;: [],\n\t\&quot;Content\&quot;: {\n\t\t\&quot;Body\&quot;: \&quot;Happy Weekend\&quot;,\n\t\t\&quot;BodyText\&quot;: \&quot;Happy Weekend\&quot;,\n\t\t\&quot;ContentType\&quot;: \&quot;text/html\&quot;,\n\t\t\&quot;Encoding\&quot;: \&quot;UTF-8\&quot;\n\t}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>8b925195-4b6b-43e7-8586-28829e4f57c9</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/api/message</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>0e58b572-a33a-40a0-9d91-2a5439598b1a</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
