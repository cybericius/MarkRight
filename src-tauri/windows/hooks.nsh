; Installer hooks: keep Windows' own "Open with" entries pointing at this install.
;
; Choosing an app through "Open with" makes Windows write per-user keys that
; store the exe's full path: `<ext>_auto_file` and `Applications\<exe>`. No
; installer owns them, so they go stale when the install folder changes (for
; example after switching between the MSI and this installer), leaving a dead
; MarkRight entry in "Open with" and blocking a new choice.
;
; Install: repair such keys whose exe no longer exists, and register
; `Applications\markright-app.exe` ourselves. Uninstall (not an update): remove
; the keys that point at this install.

!include LogicLib.nsh

!define MR_EXE "markright-app.exe"

; Runs a macro once per associated extension. Keep in sync with
; MARKDOWN_EXTENSIONS in crates/markright-core/src/files.rs (a test checks it).
!macro MR_EACH_EXT _MACRO
  !insertmacro ${_MACRO} "md"
  !insertmacro ${_MACRO} "markdown"
  !insertmacro ${_MACRO} "mdown"
  !insertmacro ${_MACRO} "mkd"
  !insertmacro ${_MACRO} "mdx"
!macroend

; $R0: a shell\open\command value. Sets $R1 to the exe path it runs when that
; exe is markright-app.exe, otherwise to "".
!macro MR_DEFINE_EXE_FROM_COMMAND UN
Function ${UN}MrExeFromCommand
  Push $R2
  Push $R3
  Push $R4
  Push $R5
  StrCpy $R1 ""
  ; Skip an opening quote
  StrCpy $R2 0
  StrCpy $R3 $R0 1
  ${If} $R3 == '"'
    StrCpy $R2 1
  ${EndIf}
  ; The path ends at the first ".exe" (StrCmp ignores case)
  StrLen $R4 $R0
  StrCpy $R3 $R2
  ${DoWhile} $R3 < $R4
    StrCpy $R5 $R0 4 $R3
    ${If} $R5 == ".exe"
      IntOp $R3 $R3 + 4
      IntOp $R3 $R3 - $R2
      StrCpy $R1 $R0 $R3 $R2
      ${Break}
    ${EndIf}
    IntOp $R3 $R3 + 1
  ${Loop}
  StrCpy $R3 $R1 "" -18
  ${If} $R3 != "\${MR_EXE}"
    StrCpy $R1 ""
  ${EndIf}
  Pop $R5
  Pop $R4
  Pop $R3
  Pop $R2
FunctionEnd
!macroend
!insertmacro MR_DEFINE_EXE_FROM_COMMAND ""
!insertmacro MR_DEFINE_EXE_FROM_COMMAND "un."

; Point a key at this install when it runs a markright-app.exe that is gone.
!macro MR_REPAIR_KEY KEY
  ReadRegStr $R0 HKCU "Software\Classes\${KEY}\shell\open\command" ""
  Call MrExeFromCommand
  ${If} $R1 != ""
  ${AndIfNot} ${FileExists} $R1
    WriteRegStr HKCU "Software\Classes\${KEY}\shell\open\command" "" '"$INSTDIR\${MR_EXE}" "%1"'
  ${EndIf}
!macroend

; Delete a key from ROOT when it runs this install's exe.
!macro MR_REMOVE_KEY ROOT KEY
  ReadRegStr $R0 ${ROOT} "Software\Classes\${KEY}\shell\open\command" ""
  Call un.MrExeFromCommand
  ${If} $R1 == "$INSTDIR\${MR_EXE}"
    DeleteRegKey ${ROOT} "Software\Classes\${KEY}"
  ${EndIf}
!macroend

!macro MR_REPAIR_AUTO_FILE EXT
  !insertmacro MR_REPAIR_KEY "${EXT}_auto_file"
!macroend

!macro MR_SUPPORTED_TYPE EXT
  WriteRegStr SHCTX "Software\Classes\Applications\${MR_EXE}\SupportedTypes" ".${EXT}" ""
!macroend

!macro MR_REMOVE_AUTO_FILE EXT
  !insertmacro MR_REMOVE_KEY HKCU "${EXT}_auto_file"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  !insertmacro MR_EACH_EXT MR_REPAIR_AUTO_FILE
  !insertmacro MR_REPAIR_KEY "Applications\${MR_EXE}"
  ; Our own "Open with" registration, so choosing the exe always finds this install
  WriteRegStr SHCTX "Software\Classes\Applications\${MR_EXE}" "FriendlyAppName" "${PRODUCTNAME}"
  WriteRegStr SHCTX "Software\Classes\Applications\${MR_EXE}\shell\open\command" "" '"$INSTDIR\${MR_EXE}" "%1"'
  !insertmacro MR_EACH_EXT MR_SUPPORTED_TYPE
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ; An update reinstalls right after, so keep the user's choices
  ${If} $UpdateMode <> 1
    !insertmacro MR_EACH_EXT MR_REMOVE_AUTO_FILE
    !insertmacro MR_REMOVE_KEY HKCU "Applications\${MR_EXE}"
    !insertmacro MR_REMOVE_KEY SHCTX "Applications\${MR_EXE}"
  ${EndIf}
!macroend
