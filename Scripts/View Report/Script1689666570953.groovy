import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Search - Global'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - All/a_General Dashboard'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - General/a_Report Case'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - General/a_Case Detail'))

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - Report C_0e0f7a/button_Search_advbtninbound_table_010'))

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - Report C_0e0f7a/a_Interaction'))

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - Report C_0e0f7a/a_History Interaction'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - Report H_df9c07/button_Search_advbtn'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - Report H_df9c07/a_Report SLA'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - Report Sla/button_Search_advbtnslaagent_table_010'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - Report Sla/a_Mapping Case'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - Report Sla/a_Number'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - Report M_78d24f/button_Search_advbtncategory_table_010'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - Report M_78d24f/a_Graphic'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - Mapping Case/a_Trend'))

WebUI.click(findTestObject('Object Repository/Report/Page_Omni Site - OneCloud - Case - Trend/a_Agent Activity'))

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - Report A_bb80b7/button_Search_advbtntab_report_agent_activity'))

WebUI.takeScreenshot()