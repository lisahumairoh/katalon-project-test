<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>send HSM waba</name>
   <tag></tag>
   <elementGuidId>c7412fc3-ec29-4277-aa30-386efff20b00</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;ConnectionId\&quot;: 588,\n\t\&quot;To\&quot;: \&quot;6285781255262\&quot;,\n\t\&quot;Name\&quot;: \&quot;Dingirl\&quot;,\n\t\&quot;Content\&quot;: {\n\t\t\&quot;Params\&quot;: [\&quot;Dingirl\&quot;]\n\t}\n}&quot;,
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
      <webElementGuid>44173591-0250-47d6-886f-8b9610ccf2c3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/api/message/sendHsm/informasi_reschedule</restUrl>
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
      <id>b778ae59-c91b-4d93-b7d6-c41d2030538d</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
