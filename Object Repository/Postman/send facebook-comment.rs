<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>send facebook-comment</name>
   <tag></tag>
   <elementGuidId>04ef77a8-ee15-4d2d-9bc4-48341c59eba9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;ConnectionId\&quot;: 666,\n\t\&quot;Subject\&quot;: \&quot;good afternoon :)\&quot;,\n\t\&quot;MethodId\&quot;: \&quot;COMMENT\&quot;,\n\t\&quot;From\&quot;: \&quot;427622697612725\&quot;,\n\t\&quot;To\&quot;: \&quot;4172228609470373\&quot;,\n\t\&quot;PreviousId\&quot;: 98122,\n\t\&quot;Attachments\&quot;: [],\n\t\&quot;Content\&quot;: {\n\t\t\&quot;Body\&quot;: \&quot;:)\&quot;,\n\t\t\&quot;BodyText\&quot;: \&quot;:)\&quot;,\n\t\t\&quot;ContentType\&quot;: \&quot;text/plain\&quot;,\n\t\t\&quot;Encoding\&quot;: \&quot;UTF-8\&quot;\n\t}\n}&quot;,
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
      <webElementGuid>cb8ecbad-9996-4716-a70f-f6039a523bc3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/api/v1/message</restUrl>
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
      <id>8db81527-587c-459f-bc81-645fac70aa95</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
