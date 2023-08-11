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

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - All/a_General Dashboard'))

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/button_refresh_advbtnvr1zp'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/button_This Month'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/button_This Month'))

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/li_Last 3 Month'))

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/button_Search'))

WebUI.rightClick(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/button_refresh_advbtnvr1zp'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/span_refresh_oicon s12 no-margins'))

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/span_refresh_oicon s12 no-margins'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/div_Filter'), 0)

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/i_Filter_fa fa-angle-down'))

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/li_This Month'))

WebUI.click(findTestObject('Object Repository/Page_Omni Site - OneCloud - Case - General/button_Search'))

WebUI.closeBrowser()

