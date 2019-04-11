<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetToken</name>
   <tag></tag>
   <elementGuidId>1d4d0955-0b2b-426b-95d1-13ae16cba2f2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${server}/manage/server/generatetoken?password=${pwd}&amp;value=${v}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'Xlion061'</defaultValue>
      <description></description>
      <id>da92fe0f-795d-40f2-9734-9b3c8defc250</id>
      <masked>false</masked>
      <name>pwd</name>
   </variables>
   <variables>
      <defaultValue>'~31032056275O4Z5~ufc'</defaultValue>
      <description></description>
      <id>1de4e67a-a0e5-4672-9c28-5c0da90b5727</id>
      <masked>false</masked>
      <name>v</name>
   </variables>
   <variables>
      <defaultValue>'http://172.16.0.67:81/personalization'</defaultValue>
      <description></description>
      <id>ca7ffaa4-d2fd-4ee6-95bd-718a837b2ec0</id>
      <masked>false</masked>
      <name>server</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable


RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



def jsonSlurper = new JsonSlurper()
//transfer result to json
def jsonResponse = jsonSlurper.parseText(response.getResponseText())
//get token from json result
def token = jsonResponse.token
//write into csv
File file = new File(&quot;Data Files/testData/token.csv&quot;)
BufferedWriter out = new BufferedWriter(new FileWriter(file))
out.write(token)
out.flush();
out.close();


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
