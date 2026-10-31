; Standalone uninstall removes all per-user TutorLoL state, independent of the checkbox.
; The vendored replacement installer always invokes the old uninstaller with /UPDATE.
!macro NSIS_HOOK_POSTUNINSTALL
  ${If} $UpdateMode <> 1
    SetShellVarContext current
    ClearErrors
    RMDir /r "$LOCALAPPDATA\${BUNDLEID}"
    RMDir /r "$APPDATA\${BUNDLEID}"
    ; Check the actual paths after removal, rather than treating absent dirs as failures.
    ${If} ${FileExists} "$LOCALAPPDATA\${BUNDLEID}\*.*"
    ${OrIf} ${FileExists} "$APPDATA\${BUNDLEID}\*.*"
      DetailPrint "TutorLoL data cleanup failed: close the app and retry uninstall."
      MessageBox MB_OK|MB_ICONEXCLAMATION "TutorLoL could not remove all app data. Close the app and its windows, then remove the remaining TutorLoL app data before reinstalling." /SD IDOK
      SetErrorLevel 2
      Quit
    ${EndIf}
  ${EndIf}
!macroend
