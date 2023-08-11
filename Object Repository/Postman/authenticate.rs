<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>authenticate</name>
   <tag></tag>
   <elementGuidId>285c7645-053e-4f90-9bfd-f278c45b058c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;strategy&quot;,
      &quot;value&quot;: &quot;local&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;email&quot;,
      &quot;value&quot;: &quot;dinni@ciptadrasoft.com&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;dingirl&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;siteId&quot;,
      &quot;value&quot;: &quot;169&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/api/authenticate</restUrl>
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
      <id>9d4eef51-3c95-4d1c-94a4-ab6fcdcb9e3f</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
