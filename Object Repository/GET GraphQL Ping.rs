<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Getting GraphQL data using GET method.</description>
   <name>GET GraphQL Ping</name>
   <tag></tag>
   <elementGuidId>ea06ce07-04e1-4d71-b95f-bb1fe73ad911</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.url}?query=query Query{ characters(filter: { name: &quot;${GlobalVariable.name1}&quot; }) { results { id name status species type gender } } locations { results { id name dimension } } episodes { results { id name air_date episode } } }</restUrl>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


String jsonPass =
&quot;&quot;&quot;
{
  &quot;\$schema&quot;: &quot;http://json-schema.org/draft-07/schema#&quot;,
  &quot;title&quot;: &quot;Generated schema for Root&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;data&quot;: {
      &quot;type&quot;: &quot;object&quot;,
      &quot;properties&quot;: {
        &quot;characters&quot;: {
          &quot;type&quot;: &quot;object&quot;,
          &quot;properties&quot;: {
            &quot;info&quot;: {
              &quot;type&quot;: &quot;object&quot;,
              &quot;properties&quot;: {
                &quot;count&quot;: {
                  &quot;type&quot;: [&quot;number&quot;, &quot;null&quot;]
                },
                &quot;pages&quot;: {
                  &quot;type&quot;: [&quot;number&quot;, &quot;null&quot;]
                },
                &quot;next&quot;: {
                  &quot;type&quot;: [&quot;number&quot;, &quot;null&quot;]
                },
                &quot;prev&quot;: {}
              },
              &quot;optional&quot;: [
                &quot;count&quot;,
                &quot;pages&quot;,
                &quot;next&quot;,
                &quot;prev&quot;
              ]
            },
            &quot;results&quot;: {
              &quot;type&quot;: &quot;array&quot;,
              &quot;items&quot;: {
                &quot;type&quot;: &quot;object&quot;,
                &quot;properties&quot;: {
                  &quot;id&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;name&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;status&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;species&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;type&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;gender&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;origin&quot;: {
                    &quot;type&quot;: &quot;object&quot;,
                    &quot;properties&quot;: {
                      &quot;id&quot;: {
                        &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                      },
                      &quot;name&quot;: {
                        &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                      },
                      &quot;residents&quot;: {
                        &quot;type&quot;: &quot;array&quot;,
                        &quot;items&quot;: {
                          &quot;type&quot;: &quot;object&quot;,
                          &quot;properties&quot;: {
                            &quot;id&quot;: {
                              &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                            },
                            &quot;name&quot;: {
                              &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                            }
                          },
                          &quot;optional&quot;: [
                            &quot;id&quot;,
                            &quot;name&quot;
                          ]
                        }
                      }
                    },
                    &quot;optional&quot;: [
                      &quot;name&quot;,
                      &quot;residents&quot;
                    ]
                  },
                  &quot;image&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;created&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  }
                },
                &quot;optional&quot;: [
                  &quot;id&quot;,
                  &quot;name&quot;,
                  &quot;status&quot;,
                  &quot;species&quot;,
                  &quot;type&quot;,
                  &quot;gender&quot;,
                  &quot;origin&quot;,
                  &quot;image&quot;,
                  &quot;created&quot;
                ]
              }
            }
          },
          &quot;optional&quot;: [
            &quot;info&quot;,
            &quot;results&quot;
          ]
        },
        &quot;locations&quot;: {
          &quot;type&quot;: &quot;object&quot;,
          &quot;properties&quot;: {
            &quot;info&quot;: {
              &quot;type&quot;: &quot;object&quot;,
              &quot;properties&quot;: {
                &quot;count&quot;: {
                  &quot;type&quot;: [&quot;number&quot;, &quot;null&quot;]
                },
                &quot;pages&quot;: {
                  &quot;type&quot;: [&quot;number&quot;, &quot;null&quot;]
                },
                &quot;next&quot;: {
                  &quot;type&quot;: [&quot;number&quot;, &quot;null&quot;]
                },
                &quot;prev&quot;: {
                  &quot;type&quot;: [&quot;number&quot;, &quot;null&quot;]
                }
              },
              &quot;optional&quot;: [
                &quot;count&quot;,
                &quot;pages&quot;,
                &quot;next&quot;,
                &quot;prev&quot;
              ]
            },
            &quot;results&quot;: {
              &quot;type&quot;: &quot;array&quot;,
              &quot;items&quot;: {
                &quot;type&quot;: &quot;object&quot;,
                &quot;properties&quot;: {
                  &quot;id&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;name&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;type&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;dimension&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;created&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;residents&quot;: {
                    &quot;type&quot;: &quot;array&quot;,
                    &quot;items&quot;: {
                      &quot;type&quot;: &quot;object&quot;,
                      &quot;properties&quot;: {
                        &quot;id&quot;: {
                          &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                        },
                        &quot;name&quot;: {
                          &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                        }
                      },
                      &quot;optional&quot;: [
                        &quot;id&quot;,
                        &quot;name&quot;
                      ]
                    }
                  }
                },
                &quot;optional&quot;: [
                  &quot;id&quot;,
                  &quot;name&quot;,
                  &quot;type&quot;,
                  &quot;dimension&quot;,
                  &quot;created&quot;,
                  &quot;residents&quot;
                ]
              }
            }
          },
          &quot;optional&quot;: [
            &quot;info&quot;,
            &quot;results&quot;
          ]
        },
        &quot;episodes&quot;: {
          &quot;type&quot;: &quot;object&quot;,
          &quot;properties&quot;: {
            &quot;info&quot;: {
              &quot;type&quot;: &quot;object&quot;,
              &quot;properties&quot;: {
                &quot;count&quot;: {
                  &quot;type&quot;: [&quot;number&quot;, &quot;null&quot;]
                },
                &quot;pages&quot;: {
                  &quot;type&quot;: [&quot;number&quot;, &quot;null&quot;]
                },
                &quot;next&quot;: {
                  &quot;type&quot;: [&quot;number&quot;, &quot;null&quot;]
                },
                &quot;prev&quot;: {}
              },
              &quot;optional&quot;: [
                &quot;count&quot;,
                &quot;pages&quot;,
                &quot;next&quot;,
                &quot;prev&quot;
              ]
            },
            &quot;results&quot;: {
              &quot;type&quot;: &quot;array&quot;,
              &quot;items&quot;: {
                &quot;type&quot;: &quot;object&quot;,
                &quot;properties&quot;: {
                  &quot;id&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;name&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;air_date&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;episode&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  },
                  &quot;characters&quot;: {
                    &quot;type&quot;: &quot;array&quot;,
                    &quot;items&quot;: {
                      &quot;type&quot;: &quot;object&quot;,
                      &quot;properties&quot;: {
                        &quot;id&quot;: {
                          &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                        },
                        &quot;name&quot;: {
                          &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                        }
                      },
                      &quot;optional&quot;: [
                        &quot;id&quot;,
                        &quot;name&quot;
                      ]
                    }
                  },
                  &quot;created&quot;: {
                    &quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]
                  }
                },
                &quot;optional&quot;: [
                  &quot;id&quot;,
                  &quot;name&quot;,
                  &quot;air_date&quot;,
                  &quot;episode&quot;,
                  &quot;characters&quot;,
                  &quot;created&quot;
                ]
              }
            }
          },
          &quot;optional&quot;: [
            &quot;info&quot;,
            &quot;results&quot;
          ]
        }
      },
      &quot;optional&quot;: [
        &quot;characters&quot;,
        &quot;locations&quot;,
        &quot;episodes&quot;
      ]
    }
  },
  &quot;optional&quot;: [
    &quot;data&quot;
  ]
}
&quot;&quot;&quot;
boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
