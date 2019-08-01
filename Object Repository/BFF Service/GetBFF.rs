<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetBFF</name>
   <tag></tag>
   <elementGuidId>49164109-66eb-46ba-99fb-9e845320d178</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>platform</name>
      <type>Main</type>
      <value>Android</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>language</name>
      <type>Main</type>
      <value>es</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6IjI4Y2M2MzEyZWVkYjI1MzIwMDQyMjI4MWE4MTQ4N2UyYTkzMjJhOTIiLCJ0eXAiOiJKV1QifQ.eyJwaG90b1VSTCI6Imh0dHA6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL0tUOGVCY0U2ZmRRaC02c3Q1UVZTckQxRk1Vck5lZVR0UjlibmFOQnVja0VGdDh3VGs4T092QmVPZTQ5Zkc2UU1EbGxNZS1zc3dWQmlHVkx4QlBRNFRRIiwicm9sZSI6IkNhbmRpZGF0ZSIsImFkZHJlc3MiOiJCb2dvdMOhLCBCb2dvdGEiLCJkaXNwbGF5TmFtZSI6IkNhbmRwcm8xIERhYiAzIiwibGF0aXR1ZGUiOjQuNjYxMDY2NTMyMTM1MDEsImxvbmdpdHVkZSI6LTc0LjA0NzYzNzkzOTQ1MzEyLCJpc3MiOiJodHRwczovL3NlY3VyZXRva2VuLmdvb2dsZS5jb20vbWVybGluLXBybyIsImF1ZCI6Im1lcmxpbi1wcm8iLCJhdXRoX3RpbWUiOjE1NjQ2MTM5ODQsInVzZXJfaWQiOiJmNmY5MmY5My03MmE3LTQzNjItYjJmMC00OGU2Y2Y5MzlkZjMiLCJzdWIiOiJmNmY5MmY5My03MmE3LTQzNjItYjJmMC00OGU2Y2Y5MzlkZjMiLCJpYXQiOjE1NjQ2MTgzODYsImV4cCI6MTU2NDYyMTk4NiwiZW1haWwiOiJjYW5kcHJvMUBtZXJsaW5qb2JzLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjpmYWxzZSwiZmlyZWJhc2UiOnsiaWRlbnRpdGllcyI6eyJlbWFpbCI6WyJjYW5kcHJvMUBtZXJsaW5qb2JzLmNvbSJdfSwic2lnbl9pbl9wcm92aWRlciI6ImN1c3RvbSJ9fQ.AHvyAoA-MLuo6VmVFRwKOdUjNOj8VipKiV2uA-c9k6ZErqQU7L23jvucdH90mFr368g4himE8qvyVeBqOGf7dlNFDY89jB2YeDNvRQAqj17YvTFVx9HPvPIn1m-kSx2zlCXxD6luCyAPWGaoUUNbo1TnAlr8ybFhr6bFBpo97UwDkK3yDAt3HZV-IXUbqoJn4jvnkV5A44E8pwirsVWI5ALWnhkHwSl0CP7k_vcaE0pWHCHoHWb-27Kz3oL5LOVLKFX0qZEpN3cHMQdAMcupKD03paS9i3as5bSuk2eFcw-Bkd1mdmOLH69Ytg7tDlfEI37_F6albyRRhJTvdlOACA</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.merlinjobs.com/bff/v1/candidate/chat/sortBy?status=INACTIVE&amp;lastRequestDate=0&amp;sortBy=newest</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)




WS.verifyElementPropertyValue(response, 'jobs[0].initials', 'LR')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
