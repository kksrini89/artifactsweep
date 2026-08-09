; ArtifactSweep — User PATH for sweep CLI (simple, Dist-3)
!include "WinMessages.nsh"

!macro NSIS_HOOK_POSTINSTALL
  DetailPrint "Adding install directory to user PATH..."
  ReadRegStr $0 HKCU "Environment" "Path"
  ${If} $0 == ""
    WriteRegExpandStr HKCU "Environment" "Path" "$INSTDIR"
  ${Else}
    WriteRegExpandStr HKCU "Environment" "Path" "$0;$INSTDIR"
  ${EndIf}
  SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ; PATH cleanup on uninstall can be hardened later (needs un.* helpers).
  DetailPrint "Uninstall: if 'sweep' remains on PATH, remove the ArtifactSweep folder from User environment Path."
!macroend