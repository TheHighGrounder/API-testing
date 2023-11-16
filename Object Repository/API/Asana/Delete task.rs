<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Delete task</name>
   <tag></tag>
   <elementGuidId>c1a2eda7-02eb-4ecb-96b0-b4a8139e745f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:type</name>
      <type>Main</type>
      <value>OAuth 2.0</value>
      <webElementGuid>79a69e6c-3b0a-4133-9f55-ce1068d12334</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer 2/1205959188539643/1205959197790745:ff333b52310f4b879267d34faa3f4654</value>
      <webElementGuid>f3e0b8ee-c3f6-4919-8eb1-b200957fdc45</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:grant_type</name>
      <type>Main</type>
      <value>Password Credentials</value>
      <webElementGuid>60fb55be-b4f5-4194-95c3-0365daff87c2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:access_token_url</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>337ba967-4a02-4b06-bc4f-a1bed2772273</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:state</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>7a627c5d-e39c-4c54-b6a7-ec194c2c0b27</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:authorization_code</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>405bffb3-775b-4aa2-836e-a1eb47cb60e4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:scope</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>40f1d27a-f2dc-4854-9103-a925478329dd</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:access</name>
      <type>Main</type>
      <value>2/1205959188539643/1205959197790745:ff333b52310f4b879267d34faa3f4654</value>
      <webElementGuid>1726f392-56d3-4930-a16d-4bf74fa01adc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:refesh_token</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>f00ab268-385c-4dbe-b9ce-475a8a41d23b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:oauth_consumer_key</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>5615fe7f-dc52-4ec7-a319-7acefa89b463</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:oauth_consumer_secret</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>8d881129-4a5d-428d-ab76-be980d737429</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:token_type</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>ad317d71-6ac3-468f-9121-ccf8f2bf3768</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:user_name</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>685222b9-480a-40a1-b2b7-3eb9dcd40116</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization:password</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>7beac00e-e8c5-4be1-a0b4-266e3c840be5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>https://app.asana.com/api/1.0/tasks/1205959931371800</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
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
