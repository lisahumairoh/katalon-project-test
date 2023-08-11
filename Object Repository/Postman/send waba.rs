<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>send waba</name>
   <tag></tag>
   <elementGuidId>74ba6fab-2028-4c9f-af60-e116a9d0ea2b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;ConnectionId\&quot;: 588,\n\t\&quot;Subject\&quot;: \&quot;hello\&quot;,\n\t\&quot;From\&quot;: \&quot;628111020197\&quot;,\n\t\&quot;To\&quot;: \&quot;6285781255262\&quot;,\n\t\&quot;PriorityValue\&quot;: 0,\n\t\&quot;Attachments\&quot;: [],\n\t\&quot;Users\&quot;: [{\n\t\t\t\&quot;IdentityId\&quot;: \&quot;628111020197\&quot;,\n\t\t\t\&quot;Name\&quot;: \&quot;Onebox\&quot;,\n\t\t\t\&quot;RoleId\&quot;: \&quot;MSR1\&quot;\n\t}, {\n\t\t\t\&quot;IdentityId\&quot;: \&quot;6285781255262\&quot;,\n\t\t\t\&quot;Name\&quot;: \&quot;Dinni D\&quot;,\n\t\t\t\&quot;RoleId\&quot;: \&quot;MSR2\&quot;\n\t}],\n\t\&quot;Content\&quot;: {\n\t\t\&quot;Body\&quot;: \&quot;:)\&quot;,\n\t\t\&quot;BodyText\&quot;: \&quot;:D\&quot;,\n\t\t\&quot;ContentType\&quot;: \&quot;text/html\&quot;,\n\t\t\&quot;Encoding\&quot;: \&quot;UTF-8\&quot;\n\t}\n}&quot;,
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
      <webElementGuid>72873854-4dba-4fc1-9766-5c51dd585ce1</webElementGuid>
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
      <id>57d8e6d2-4214-4240-8967-e79c45b7ff3b</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
