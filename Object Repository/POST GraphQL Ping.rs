<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Getting GraphQL data using POST method.</description>
   <name>POST GraphQL Ping</name>
   <tag></tag>
   <elementGuidId>58aea0c8-70d3-4d6f-8655-c6d0f860c617</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;query { characters(filter: { name: \\\&quot;morty\\\&quot; }) { results { name status } } locations { results { id name dimension } } episodes { results { id episode } } }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;query {\n  characters(filter: { name: \&quot;morty\&quot; }) {\n    results {\n      name\n      status\n    }\n  }\n  locations {\n    results {\n      id\n      name\n      dimension\n    }\n  }\n  episodes {\n    results {\n      id\n      episode\n    }\n  }\n}&quot;,
  &quot;displayVariables&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>7861b1fe-143f-40f5-bc0a-1b2a12be3817</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.url}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
