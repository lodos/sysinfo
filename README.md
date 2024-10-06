# Completely very fast RUST syysinfo project (faster than GO similar project 12 times)
## Mac OS X m1

```shell
cd /path/to/project
cargo run
```

Go to browser and you'll see completely json with system info
```shell
http://127.0.0.1:8989/system_info
```

<details>
<summary>Example output</summary>
```json
{
  "memory": {
    "total": 32768,
    "used": 32722.296875,
    "available": 45.703125
  },
  "disks": [
    {
      "name": "Macintosh HD",
      "total_space": 948584.16015625,
      "used_space": 743193.9323177338
    },
    {
      "name": "Macintosh HD",
      "total_space": 948584.16015625,
      "used_space": 743193.9323177338
    },
    {
      "name": "AntiGravity",
      "total_space": 122080.125,
      "used_space": 80.75
    },
    {
      "name": "DataStorage2TB",
      "total_space": 1907601.046875,
      "used_space": 742737.3687620163
    },
    {
      "name": "Scretcher",
      "total_space": 476611.984375,
      "used_space": 330080.87890625
    },
    {
      "name": "DataStorage10TB",
      "total_space": 9537207.953125,
      "used_space": 4644543.49373436
    }
  ],
  "cpu": [
    {
      "id": 0,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 1,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 2,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 3,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 4,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 5,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 6,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 7,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 8,
      "name": "Apple M1 Max",
      "frequency": 0
    },
    {
      "id": 9,
      "name": "Apple M1 Max",
      "frequency": 0
    }
  ],
  "video": [
    {
      "details": {
        "Apple M1 Max": {
          "Total Number of Cores": "32",
          "Displays": "",
          "Vendor": "Apple (0x106b)",
          "Chipset Model": "Apple M1 Max",
          "Metal Support": "Metal 3",
          "Online": "Yes",
          "Color LCD": "",
          "Main Display": "Yes",
          "Display Type": "Built-in Liquid Retina XDR Display",
          "Connection Type": "Internal",
          "Bus": "Built-In",
          "Automatically Adjust Brightness": "Yes",
          "Resolution": "3456 x 2234 Retina",
          "Mirror": "Off",
          "Rotation": "Supported",
          "UI Looks like": "3440 x 1440 @ 85.00Hz",
          "LG ULTRAGEAR": "",
          "Type": "GPU"
        }
      }
    }
  ],
  "displays": [
    {
      "details": {
        "Built-in Liquid Retina XDR Display": {
          "Connection Type": "Internal",
          "Resolution": "3456 x 2234 Retina",
          "Mirror": "Off",
          "Display Type": "Built-in Liquid Retina XDR Display",
          "Automatically Adjust Brightness": "Yes",
          "Online": "Yes"
        }
      }
    }
  ],
  "processes": [
    {
      "pid": 3043,
      "name": "pkd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3487,
      "name": "dataaccessd",
      "status": "Run"
    },
    {
      "pid": 987,
      "name": "pbs",
      "status": "Run"
    },
    {
      "pid": 319,
      "name": "com.apple.geod",
      "status": "Unknown(0)"
    },
    {
      "pid": 1375,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 9826,
      "name": "devicecheckd",
      "status": "Run"
    },
    {
      "pid": 1116,
      "name": "CategoriesService",
      "status": "Run"
    },
    {
      "pid": 26549,
      "name": "zsh",
      "status": "Run"
    },
    {
      "pid": 3581,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 1284,
      "name": "storedownloadd",
      "status": "Run"
    },
    {
      "pid": 2384,
      "name": "com.apple.ctkpcscd",
      "status": "Run"
    },
    {
      "pid": 799,
      "name": "akd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3149,
      "name": "ChatGPT",
      "status": "Run"
    },
    {
      "pid": 65655,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 35666,
      "name": "webinspectord",
      "status": "Run"
    },
    {
      "pid": 31093,
      "name": "org.sparkle-project.InstallerStatus",
      "status": "Run"
    },
    {
      "pid": 724,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 37663,
      "name": "Python",
      "status": "Run"
    },
    {
      "pid": 688,
      "name": "systemsoundserverd",
      "status": "Unknown(0)"
    },
    {
      "pid": 390,
      "name": "trustd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3475,
      "name": "MauiAUSP",
      "status": "Run"
    },
    {
      "pid": 4812,
      "name": "storeaccountd",
      "status": "Run"
    },
    {
      "pid": 731,
      "name": "com.apple.MobileSoftwareUpdate.CleanupPreparePathService",
      "status": "Unknown(0)"
    },
    {
      "pid": 37113,
      "name": "Python",
      "status": "Run"
    },
    {
      "pid": 904,
      "name": "rapportd",
      "status": "Run"
    },
    {
      "pid": 222,
      "name": "backupd-helper",
      "status": "Unknown(0)"
    },
    {
      "pid": 998,
      "name": "UserNotificationCenter",
      "status": "Run"
    },
    {
      "pid": 30270,
      "name": "LinkedNotesUIService",
      "status": "Run"
    },
    {
      "pid": 3908,
      "name": "logd_reporter",
      "status": "Unknown(0)"
    },
    {
      "pid": 1103,
      "name": "WiFiVelocityAgent",
      "status": "Run"
    },
    {
      "pid": 31097,
      "name": "QuickLookUIService",
      "status": "Run"
    },
    {
      "pid": 906,
      "name": "routined",
      "status": "Run"
    },
    {
      "pid": 9888,
      "name": "PodcastContentService",
      "status": "Run"
    },
    {
      "pid": 713,
      "name": "com.apple.AppleUserHIDDrivers",
      "status": "Unknown(0)"
    },
    {
      "pid": 152,
      "name": "keybagd",
      "status": "Unknown(0)"
    },
    {
      "pid": 570,
      "name": "XProtectPluginService",
      "status": "Unknown(0)"
    },
    {
      "pid": 928,
      "name": "tccd",
      "status": "Run"
    },
    {
      "pid": 86289,
      "name": "WiFiCloudAssetsXPCService",
      "status": "Unknown(0)"
    },
    {
      "pid": 69919,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 19752,
      "name": "rcd",
      "status": "Run"
    },
    {
      "pid": 3817,
      "name": "com.apple.MobileInstallationHelperService",
      "status": "Unknown(0)"
    },
    {
      "pid": 257,
      "name": "hidd",
      "status": "Unknown(0)"
    },
    {
      "pid": 19026,
      "name": "translationd",
      "status": "Run"
    },
    {
      "pid": 2200,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 304,
      "name": "sbis3plugin",
      "status": "Unknown(0)"
    },
    {
      "pid": 1369,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 49838,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 17857,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 20033,
      "name": "Notes",
      "status": "Run"
    },
    {
      "pid": 958,
      "name": "fontd",
      "status": "Run"
    },
    {
      "pid": 186,
      "name": "dirhelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 51829,
      "name": "periodic-wrapper",
      "status": "Unknown(0)"
    },
    {
      "pid": 3454,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 915,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 132,
      "name": "UserEventAgent",
      "status": "Unknown(0)"
    },
    {
      "pid": 3910,
      "name": "FPCKService",
      "status": "Run"
    },
    {
      "pid": 668,
      "name": "ctkahp",
      "status": "Unknown(0)"
    },
    {
      "pid": 248,
      "name": "PlugInLibraryService",
      "status": "Unknown(0)"
    },
    {
      "pid": 1020,
      "name": "WiFiAgent",
      "status": "Run"
    },
    {
      "pid": 61422,
      "name": "PerfPowerServices",
      "status": "Unknown(0)"
    },
    {
      "pid": 1081,
      "name": "askpermissiond",
      "status": "Run"
    },
    {
      "pid": 35668,
      "name": "RemotePairingDataVaultHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 9830,
      "name": "IMAutomaticHistoryDeletionAgent",
      "status": "Run"
    },
    {
      "pid": 3496,
      "name": "proactived",
      "status": "Run"
    },
    {
      "pid": 35700,
      "name": "QuickLookUIService",
      "status": "Run"
    },
    {
      "pid": 1119,
      "name": "CopyClip 2",
      "status": "Run"
    },
    {
      "pid": 69913,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 1085,
      "name": "AirPlayUIAgent",
      "status": "Run"
    },
    {
      "pid": 21631,
      "name": "Bitrix24 Helper (GPU)",
      "status": "Run"
    },
    {
      "pid": 1286,
      "name": "cloudd",
      "status": "Unknown(0)"
    },
    {
      "pid": 914,
      "name": "siriactionsd",
      "status": "Run"
    },
    {
      "pid": 388,
      "name": "secinitd",
      "status": "Unknown(0)"
    },
    {
      "pid": 950,
      "name": "siriknowledged",
      "status": "Run"
    },
    {
      "pid": 356,
      "name": "bootinstalld",
      "status": "Unknown(0)"
    },
    {
      "pid": 3038,
      "name": "fairplayd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3472,
      "name": "SiriAUSP",
      "status": "Run"
    },
    {
      "pid": 869,
      "name": "securityd_service",
      "status": "Unknown(0)"
    },
    {
      "pid": 9847,
      "name": "ASConfigurationSubscriber",
      "status": "Run"
    },
    {
      "pid": 956,
      "name": "FinderSyncExtension",
      "status": "Run"
    },
    {
      "pid": 1158,
      "name": "countryd",
      "status": "Unknown(0)"
    },
    {
      "pid": 36007,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 3545,
      "name": "ManagedConfigurationFilesSubscriber",
      "status": "Unknown(0)"
    },
    {
      "pid": 1035,
      "name": "UsageTrackingAgent",
      "status": "Run"
    },
    {
      "pid": 9842,
      "name": "PasscodeSettingsSubscriber",
      "status": "Run"
    },
    {
      "pid": 4005,
      "name": "VTDecoderXPCService",
      "status": "Run"
    },
    {
      "pid": 9841,
      "name": "LegacyProfilesSubscriber",
      "status": "Run"
    },
    {
      "pid": 2204,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 22189,
      "name": "Installer",
      "status": "Run"
    },
    {
      "pid": 21623,
      "name": "Bitrix24",
      "status": "Run"
    },
    {
      "pid": 879,
      "name": "seld",
      "status": "Unknown(0)"
    },
    {
      "pid": 5881,
      "name": "ciphermld",
      "status": "Run"
    },
    {
      "pid": 3506,
      "name": "nsattributedstringagent",
      "status": "Run"
    },
    {
      "pid": 943,
      "name": "WallpaperDynamicExtension",
      "status": "Run"
    },
    {
      "pid": 1044,
      "name": "Google Chrome",
      "status": "Run"
    },
    {
      "pid": 50172,
      "name": "netbiosd",
      "status": "Unknown(0)"
    },
    {
      "pid": 72832,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 929,
      "name": "iconservicesagent",
      "status": "Run"
    },
    {
      "pid": 910,
      "name": "usernotificationsd",
      "status": "Run"
    },
    {
      "pid": 1067,
      "name": "axassetsd",
      "status": "Run"
    },
    {
      "pid": 3050,
      "name": "containermanagerd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1047,
      "name": "spindump_agent",
      "status": "Run"
    },
    {
      "pid": 891,
      "name": "BackgroundTaskManagementAgent",
      "status": "Run"
    },
    {
      "pid": 4094,
      "name": "cloudphotod",
      "status": "Run"
    },
    {
      "pid": 384,
      "name": "suhelperd",
      "status": "Unknown(0)"
    },
    {
      "pid": 974,
      "name": "sharingd",
      "status": "Run"
    },
    {
      "pid": 386,
      "name": "com.apple.geod",
      "status": "Unknown(0)"
    },
    {
      "pid": 3023,
      "name": "audiomxd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1016,
      "name": "imagent",
      "status": "Run"
    },
    {
      "pid": 3464,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 338,
      "name": "com.apple.ColorSyncXPCAgent",
      "status": "Unknown(0)"
    },
    {
      "pid": 332,
      "name": "containermanagerd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3548,
      "name": "appleaccountd",
      "status": "Run"
    },
    {
      "pid": 3455,
      "name": "KonaSynthesizer",
      "status": "Run"
    },
    {
      "pid": 2508,
      "name": "sbis-cef-helper",
      "status": "Run"
    },
    {
      "pid": 322,
      "name": "backupd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3466,
      "name": "CalendarFocusConfigurationExtension",
      "status": "Run"
    },
    {
      "pid": 3045,
      "name": "csnameddatad",
      "status": "Unknown(0)"
    },
    {
      "pid": 19754,
      "name": "VisualizerService",
      "status": "Run"
    },
    {
      "pid": 18095,
      "name": "userfs_helper",
      "status": "Unknown(0)"
    },
    {
      "pid": 3346,
      "name": "SiriNCService",
      "status": "Run"
    },
    {
      "pid": 3017,
      "name": "com.apple.siri-distributed-evaluation",
      "status": "Run"
    },
    {
      "pid": 960,
      "name": "ContextService",
      "status": "Run"
    },
    {
      "pid": 250,
      "name": "systemstatusd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3453,
      "name": "DiskUnmountWatcher",
      "status": "Run"
    },
    {
      "pid": 657,
      "name": "coreauthd",
      "status": "Unknown(0)"
    },
    {
      "pid": 372,
      "name": "symptomsd-diag",
      "status": "Unknown(0)"
    },
    {
      "pid": 1093,
      "name": "amsaccountsd",
      "status": "Run"
    },
    {
      "pid": 266,
      "name": "AudioComponentRegistrar",
      "status": "Unknown(0)"
    },
    {
      "pid": 9868,
      "name": "proactiveeventtrackerd",
      "status": "Run"
    },
    {
      "pid": 166,
      "name": "syslogd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1064,
      "name": "bluetoothuserd",
      "status": "Run"
    },
    {
      "pid": 3595,
      "name": "ThumbnailExtension_macOS",
      "status": "Run"
    },
    {
      "pid": 859,
      "name": "siriinferenced",
      "status": "Unknown(0)"
    },
    {
      "pid": 949,
      "name": "mobiletimerd",
      "status": "Run"
    },
    {
      "pid": 719,
      "name": "rtcreportingd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1074,
      "name": "Siri",
      "status": "Run"
    },
    {
      "pid": 992,
      "name": "suggestd",
      "status": "Run"
    },
    {
      "pid": 68409,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 1120,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 715,
      "name": "WiFiCloudAssetsXPCService",
      "status": "Unknown(0)"
    },
    {
      "pid": 1283,
      "name": "geodMachServiceBridge",
      "status": "Run"
    },
    {
      "pid": 964,
      "name": "cloudd",
      "status": "Run"
    },
    {
      "pid": 364,
      "name": "mds_stores",
      "status": "Unknown(0)"
    },
    {
      "pid": 9824,
      "name": "EscrowSecurityAlert",
      "status": "Run"
    },
    {
      "pid": 1603,
      "name": "chrome_crashpad_handler",
      "status": "Run"
    },
    {
      "pid": 725,
      "name": "UARPUpdaterServiceDisplay",
      "status": "Unknown(0)"
    },
    {
      "pid": 49841,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 71304,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 712,
      "name": "com.apple.DriverKit-IOUserDockChannelSerial",
      "status": "Unknown(0)"
    },
    {
      "pid": 195,
      "name": "corebrightnessd",
      "status": "Unknown(0)"
    },
    {
      "pid": 34544,
      "name": "intents_helper",
      "status": "Run"
    },
    {
      "pid": 72839,
      "name": "fsnotifier",
      "status": "Run"
    },
    {
      "pid": 68166,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 42286,
      "name": "microstackshot",
      "status": "Unknown(0)"
    },
    {
      "pid": 4476,
      "name": "XProtect",
      "status": "Unknown(0)"
    },
    {
      "pid": 1063,
      "name": "avconferenced",
      "status": "Run"
    },
    {
      "pid": 733,
      "name": "StandaloneHIDAudService",
      "status": "Unknown(0)"
    },
    {
      "pid": 730,
      "name": "nfcd",
      "status": "Unknown(0)"
    },
    {
      "pid": 335,
      "name": "cameracaptured",
      "status": "Unknown(0)"
    },
    {
      "pid": 188,
      "name": "revisiond",
      "status": "Unknown(0)"
    },
    {
      "pid": 953,
      "name": "awdd",
      "status": "Unknown(0)"
    },
    {
      "pid": 5299,
      "name": "MTLCompilerService",
      "status": "Unknown(0)"
    },
    {
      "pid": 1089,
      "name": "audioaccessoryd",
      "status": "Run"
    },
    {
      "pid": 1062,
      "name": "searchpartyuseragent",
      "status": "Run"
    },
    {
      "pid": 22005,
      "name": "com.apple.appkit.xpc.openAndSavePanelService",
      "status": "Run"
    },
    {
      "pid": 68348,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 75513,
      "name": "zsh",
      "status": "Run"
    },
    {
      "pid": 1095,
      "name": "FindMyWidgetItems",
      "status": "Run"
    },
    {
      "pid": 946,
      "name": "corespotlightd",
      "status": "Run"
    },
    {
      "pid": 720,
      "name": "sysmond",
      "status": "Unknown(0)"
    },
    {
      "pid": 17414,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 237,
      "name": "nehelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 3092,
      "name": "fsnotifier",
      "status": "Run"
    },
    {
      "pid": 3463,
      "name": "extensionkitservice",
      "status": "Run"
    },
    {
      "pid": 49836,
      "name": "Mail",
      "status": "Run"
    },
    {
      "pid": 346,
      "name": "aned",
      "status": "Unknown(0)"
    },
    {
      "pid": 203,
      "name": "analyticsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 155,
      "name": "watchdogd",
      "status": "Unknown(0)"
    },
    {
      "pid": 31056,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 2507,
      "name": "sbis-cef-helper",
      "status": "Run"
    },
    {
      "pid": 1453,
      "name": "automationmode-writer",
      "status": "Unknown(0)"
    },
    {
      "pid": 1038,
      "name": "AssetCacheTetheratorService",
      "status": "Unknown(0)"
    },
    {
      "pid": 9823,
      "name": "griddatad",
      "status": "Unknown(0)"
    },
    {
      "pid": 17637,
      "name": "universalAccessAuthWarn",
      "status": "Run"
    },
    {
      "pid": 18034,
      "name": "VTDecoderXPCService",
      "status": "Run"
    },
    {
      "pid": 2097,
      "name": "CVMServer",
      "status": "Unknown(0)"
    },
    {
      "pid": 223,
      "name": "PowerUIAgent",
      "status": "Unknown(0)"
    },
    {
      "pid": 35671,
      "name": "diskimagescontroller",
      "status": "Unknown(0)"
    },
    {
      "pid": 1026,
      "name": "fmfd",
      "status": "Run"
    },
    {
      "pid": 1002,
      "name": "ContainerMetadataExtractor",
      "status": "Run"
    },
    {
      "pid": 939,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 35709,
      "name": "reversetemplated",
      "status": "Run"
    },
    {
      "pid": 67616,
      "name": "datagrip",
      "status": "Run"
    },
    {
      "pid": 989,
      "name": "photolibraryd",
      "status": "Run"
    },
    {
      "pid": 51910,
      "name": "Python",
      "status": "Run"
    },
    {
      "pid": 918,
      "name": "containermanagerd",
      "status": "Run"
    },
    {
      "pid": 71698,
      "name": "com.apple.SiriTTSService.TrialProxy",
      "status": "Run"
    },
    {
      "pid": 4394,
      "name": "recentsd",
      "status": "Run"
    },
    {
      "pid": 907,
      "name": "usernoted",
      "status": "Run"
    },
    {
      "pid": 9836,
      "name": "InteractiveLegacyProfilesSubscriber",
      "status": "Run"
    },
    {
      "pid": 353,
      "name": "audioclocksyncd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3462,
      "name": "WardaSynthesizer_x86_64",
      "status": "Run"
    },
    {
      "pid": 291,
      "name": "storagekitd",
      "status": "Unknown(0)"
    },
    {
      "pid": 35657,
      "name": "ImageIOXPCService",
      "status": "Run"
    },
    {
      "pid": 2501,
      "name": "sbis-cef-helper",
      "status": "Run"
    },
    {
      "pid": 292,
      "name": "backgroundtaskmanagementd",
      "status": "Unknown(0)"
    },
    {
      "pid": 31090,
      "name": "RapidAPI",
      "status": "Run"
    },
    {
      "pid": 3541,
      "name": "PasscodeSettingsSubscriber",
      "status": "Unknown(0)"
    },
    {
      "pid": 1024,
      "name": "seserviced",
      "status": "Run"
    },
    {
      "pid": 11141,
      "name": "GameControllerConfigService",
      "status": "Unknown(0)"
    },
    {
      "pid": 189,
      "name": "KernelEventAgent",
      "status": "Unknown(0)"
    },
    {
      "pid": 3030,
      "name": "XProtectPluginService",
      "status": "Run"
    },
    {
      "pid": 896,
      "name": "CommCenter",
      "status": "Run"
    },
    {
      "pid": 36891,
      "name": "Python",
      "status": "Run"
    },
    {
      "pid": 808,
      "name": "adid",
      "status": "Unknown(0)"
    },
    {
      "pid": 395,
      "name": "com.apple.audio.Core-Audio-Driver-Service.helper",
      "status": "Unknown(0)"
    },
    {
      "pid": 358,
      "name": "ZhuGeService",
      "status": "Unknown(0)"
    },
    {
      "pid": 345,
      "name": "aneuserd",
      "status": "Unknown(0)"
    },
    {
      "pid": 17619,
      "name": "full-line-inference",
      "status": "Run"
    },
    {
      "pid": 35669,
      "name": "com.apple.WebKit.WebContent",
      "status": "Run"
    },
    {
      "pid": 5931,
      "name": "DPSubmissionService",
      "status": "Unknown(0)"
    },
    {
      "pid": 3540,
      "name": "LegacyProfilesSubscriber",
      "status": "Unknown(0)"
    },
    {
      "pid": 938,
      "name": "extensionkitservice",
      "status": "Run"
    },
    {
      "pid": 37662,
      "name": "Python",
      "status": "Run"
    },
    {
      "pid": 1282,
      "name": "appstoreagent",
      "status": "Run"
    },
    {
      "pid": 3470,
      "name": "MessagesActionExtension",
      "status": "Run"
    },
    {
      "pid": 272,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 327,
      "name": "cfprefsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 9809,
      "name": "ANEStorageMaintainer",
      "status": "Unknown(0)"
    },
    {
      "pid": 2906,
      "name": "com.apple.NRD.UpdateBrainService",
      "status": "Unknown(0)"
    },
    {
      "pid": 52753,
      "name": "STExtractionService.privileged",
      "status": "Unknown(0)"
    },
    {
      "pid": 35670,
      "name": "simdiskimaged",
      "status": "Unknown(0)"
    },
    {
      "pid": 3332,
      "name": "assistant_service",
      "status": "Run"
    },
    {
      "pid": 1025,
      "name": "IMDPersistenceAgent",
      "status": "Run"
    },
    {
      "pid": 1008,
      "name": "UIKitSystem",
      "status": "Run"
    },
    {
      "pid": 999,
      "name": "com.apple.geod",
      "status": "Run"
    },
    {
      "pid": 252,
      "name": "trustd",
      "status": "Unknown(0)"
    },
    {
      "pid": 160,
      "name": "iconservicesd",
      "status": "Unknown(0)"
    },
    {
      "pid": 148,
      "name": "amfid",
      "status": "Unknown(0)"
    },
    {
      "pid": 3502,
      "name": "fairplaydeviceidentityd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1269,
      "name": "loginitemregisterd",
      "status": "Run"
    },
    {
      "pid": 161,
      "name": "kernelmanagerd",
      "status": "Unknown(0)"
    },
    {
      "pid": 59049,
      "name": "authtrampoline",
      "status": "Unknown(0)"
    },
    {
      "pid": 9306,
      "name": "tipsd",
      "status": "Run"
    },
    {
      "pid": 68304,
      "name": "git",
      "status": "Run"
    },
    {
      "pid": 68402,
      "name": "system_info",
      "status": "Run"
    },
    {
      "pid": 68392,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 1709,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 17221,
      "name": "com.apple.geod",
      "status": "Unknown(0)"
    },
    {
      "pid": 19720,
      "name": "VTDecoderXPCService",
      "status": "Run"
    },
    {
      "pid": 9889,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 22126,
      "name": "nbstated",
      "status": "Unknown(0)"
    },
    {
      "pid": 3270,
      "name": "com.apple.audio.ComponentTagHelper",
      "status": "Run"
    },
    {
      "pid": 24938,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 1057,
      "name": "CarbonComponentScannerXPC",
      "status": "Run"
    },
    {
      "pid": 17214,
      "name": "BTLEServerAgent",
      "status": "Run"
    },
    {
      "pid": 909,
      "name": "pkd",
      "status": "Run"
    },
    {
      "pid": 871,
      "name": "corekdld",
      "status": "Unknown(0)"
    },
    {
      "pid": 52067,
      "name": "PlugInLibraryService",
      "status": "Run"
    },
    {
      "pid": 981,
      "name": "CSExattrCryptoService",
      "status": "Run"
    },
    {
      "pid": 354,
      "name": "searchpartyd",
      "status": "Unknown(0)"
    },
    {
      "pid": 170,
      "name": "opendirectoryd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3028,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 18367,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 3503,
      "name": "businessservicesd",
      "status": "Run"
    },
    {
      "pid": 3460,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 982,
      "name": "ReportCrash",
      "status": "Run"
    },
    {
      "pid": 924,
      "name": "assistantd",
      "status": "Run"
    },
    {
      "pid": 35662,
      "name": "webpushd",
      "status": "Run"
    },
    {
      "pid": 933,
      "name": "chronod",
      "status": "Run"
    },
    {
      "pid": 1097,
      "name": "PuntoSwitcher",
      "status": "Run"
    },
    {
      "pid": 57169,
      "name": "php-fpm",
      "status": "Run"
    },
    {
      "pid": 885,
      "name": "WindowManager",
      "status": "Run"
    },
    {
      "pid": 316,
      "name": "cron",
      "status": "Unknown(0)"
    },
    {
      "pid": 955,
      "name": "identityservicesd",
      "status": "Run"
    },
    {
      "pid": 192,
      "name": "bluetoothd",
      "status": "Unknown(0)"
    },
    {
      "pid": 2208,
      "name": "SafariLaunchAgent",
      "status": "Run"
    },
    {
      "pid": 1096,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 3719,
      "name": "photoanalysisd",
      "status": "Run"
    },
    {
      "pid": 65463,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 1108,
      "name": "Telegram",
      "status": "Run"
    },
    {
      "pid": 344,
      "name": "appleeventsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1021,
      "name": "Keychain Circle Notification",
      "status": "Run"
    },
    {
      "pid": 324,
      "name": "secinitd",
      "status": "Unknown(0)"
    },
    {
      "pid": 2907,
      "name": "xpcroleaccountd",
      "status": "Unknown(0)"
    },
    {
      "pid": 884,
      "name": "talagent",
      "status": "Run"
    },
    {
      "pid": 253,
      "name": "coreaudiod",
      "status": "Unknown(0)"
    },
    {
      "pid": 1010,
      "name": "nearbyd",
      "status": "Unknown(0)"
    },
    {
      "pid": 52662,
      "name": "full-line-inference",
      "status": "Run"
    },
    {
      "pid": 920,
      "name": "AMPDeviceDiscoveryAgent",
      "status": "Run"
    },
    {
      "pid": 3499,
      "name": "com.apple.hiservices-xpcservice",
      "status": "Unknown(0)"
    },
    {
      "pid": 1376,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 339,
      "name": "colorsyncd",
      "status": "Unknown(0)"
    },
    {
      "pid": 336,
      "name": "UVCAssistant",
      "status": "Unknown(0)"
    },
    {
      "pid": 9834,
      "name": "RemoteManagementAgent",
      "status": "Run"
    },
    {
      "pid": 286,
      "name": "mDNSResponder",
      "status": "Unknown(0)"
    },
    {
      "pid": 1071,
      "name": "SnagitHelper2022",
      "status": "Run"
    },
    {
      "pid": 67958,
      "name": "remindd",
      "status": "Run"
    },
    {
      "pid": 1161,
      "name": "eligibilityd",
      "status": "Unknown(0)"
    },
    {
      "pid": 18239,
      "name": "jcef Helper",
      "status": "Run"
    },
    {
      "pid": 9846,
      "name": "ManagedConfigurationFilesSubscriber",
      "status": "Run"
    },
    {
      "pid": 9818,
      "name": "ANEStorageMaintainer",
      "status": "Unknown(0)"
    },
    {
      "pid": 3535,
      "name": "InteractiveLegacyProfilesSubscriber",
      "status": "Unknown(0)"
    },
    {
      "pid": 3477,
      "name": "com.apple.SiriTTSService.TrialProxy",
      "status": "Run"
    },
    {
      "pid": 825,
      "name": "com.apple.AmbientDisplayAgent",
      "status": "Unknown(0)"
    },
    {
      "pid": 723,
      "name": "iconservicesagent",
      "status": "Unknown(0)"
    },
    {
      "pid": 3251,
      "name": "mdwrite",
      "status": "Run"
    },
    {
      "pid": 176,
      "name": "securityd",
      "status": "Unknown(0)"
    },
    {
      "pid": 980,
      "name": "neagent",
      "status": "Run"
    },
    {
      "pid": 940,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 1011,
      "name": "familycircled",
      "status": "Run"
    },
    {
      "pid": 68408,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 285,
      "name": "symptomsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 210,
      "name": "contextstored",
      "status": "Unknown(0)"
    },
    {
      "pid": 68391,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 68401,
      "name": "AXVisualSupportAgent",
      "status": "Run"
    },
    {
      "pid": 239,
      "name": "mobileassetd",
      "status": "Unknown(0)"
    },
    {
      "pid": 159,
      "name": "mds",
      "status": "Unknown(0)"
    },
    {
      "pid": 57170,
      "name": "php-fpm",
      "status": "Run"
    },
    {
      "pid": 881,
      "name": "CoreServicesUIAgent",
      "status": "Run"
    },
    {
      "pid": 3099,
      "name": "trustd",
      "status": "Unknown(0)"
    },
    {
      "pid": 204,
      "name": "loginwindow",
      "status": "Run"
    },
    {
      "pid": 1015,
      "name": "StatusKitAgent",
      "status": "Run"
    },
    {
      "pid": 37112,
      "name": "Python",
      "status": "Run"
    },
    {
      "pid": 3542,
      "name": "AccountSubscriber",
      "status": "Unknown(0)"
    },
    {
      "pid": 3051,
      "name": "XProtectBridgeService",
      "status": "Unknown(0)"
    },
    {
      "pid": 1006,
      "name": "akd",
      "status": "Run"
    },
    {
      "pid": 2904,
      "name": "installerauthagent",
      "status": "Run"
    },
    {
      "pid": 1101,
      "name": "AccessibilityVisualsAgent",
      "status": "Run"
    },
    {
      "pid": 977,
      "name": "Spotlight",
      "status": "Run"
    },
    {
      "pid": 66778,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 65941,
      "name": "com.apple.iCloudHelper",
      "status": "Run"
    },
    {
      "pid": 5286,
      "name": "com.apple.BKAgentService",
      "status": "Run"
    },
    {
      "pid": 1034,
      "name": "ndoagent",
      "status": "Run"
    },
    {
      "pid": 68389,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 742,
      "name": "trustd",
      "status": "Unknown(0)"
    },
    {
      "pid": 729,
      "name": "UARPUpdaterServiceAFU",
      "status": "Unknown(0)"
    },
    {
      "pid": 183,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 1,
      "name": "launchd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1287,
      "name": "wifivelocityd",
      "status": "Unknown(0)"
    },
    {
      "pid": 23629,
      "name": "ssh-agent",
      "status": "Run"
    },
    {
      "pid": 34703,
      "name": "com.apple.CalendarWeatherKitService",
      "status": "Run"
    },
    {
      "pid": 65862,
      "name": "Google Chrome Helper",
      "status": "Run"
    },
    {
      "pid": 3098,
      "name": "mdbulkimport",
      "status": "Unknown(0)"
    },
    {
      "pid": 1030,
      "name": "AssetCache",
      "status": "Unknown(0)"
    },
    {
      "pid": 3468,
      "name": "SafariLinkExtension",
      "status": "Run"
    },
    {
      "pid": 2001,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 193,
      "name": "notifyd",
      "status": "Unknown(0)"
    },
    {
      "pid": 978,
      "name": "com.apple.quicklook.ThumbnailsAgent",
      "status": "Run"
    },
    {
      "pid": 3792,
      "name": "PasswordBreachAgent",
      "status": "Run"
    },
    {
      "pid": 320,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 534,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 1000,
      "name": "AppSSOAgent",
      "status": "Run"
    },
    {
      "pid": 73004,
      "name": "SSTP Tunnel",
      "status": "Run"
    },
    {
      "pid": 68292,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 72831,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 3037,
      "name": "cfprefsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1066,
      "name": "ae_notifier",
      "status": "Run"
    },
    {
      "pid": 1019,
      "name": "ScreenTimeWidgetExtension",
      "status": "Run"
    },
    {
      "pid": 979,
      "name": "com.apple.CloudDocs.iCloudDriveFileProvider",
      "status": "Run"
    },
    {
      "pid": 732,
      "name": "ThunderboltAccessoryUpdaterService",
      "status": "Unknown(0)"
    },
    {
      "pid": 9783,
      "name": "periodic-wrapper",
      "status": "Unknown(0)"
    },
    {
      "pid": 2984,
      "name": "com.apple.StreamingUnzipService.privileged",
      "status": "Unknown(0)"
    },
    {
      "pid": 1023,
      "name": "ProtectedCloudKeySyncing",
      "status": "Run"
    },
    {
      "pid": 2905,
      "name": "NRDUpdated",
      "status": "Unknown(0)"
    },
    {
      "pid": 936,
      "name": "wallpaperexportd",
      "status": "Unknown(0)"
    },
    {
      "pid": 362,
      "name": "com.apple.audio.Core-Audio-Driver-Service",
      "status": "Unknown(0)"
    },
    {
      "pid": 200,
      "name": "cfprefsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 11874,
      "name": "CallHistorySyncHelper",
      "status": "Run"
    },
    {
      "pid": 931,
      "name": "ContextStoreAgent",
      "status": "Run"
    },
    {
      "pid": 34792,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 959,
      "name": "extensionkitservice",
      "status": "Run"
    },
    {
      "pid": 68412,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 3721,
      "name": "mdworker",
      "status": "Unknown(0)"
    },
    {
      "pid": 263,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 983,
      "name": "CallHistoryPluginHelper",
      "status": "Run"
    },
    {
      "pid": 2007,
      "name": "Google Chrome Helper",
      "status": "Run"
    },
    {
      "pid": 131,
      "name": "smd",
      "status": "Unknown(0)"
    },
    {
      "pid": 236,
      "name": "nsurlsessiond",
      "status": "Unknown(0)"
    },
    {
      "pid": 1105,
      "name": "findmylocateagent",
      "status": "Run"
    },
    {
      "pid": 19722,
      "name": "Termius Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 923,
      "name": "com.apple.hiservices-xpcservice",
      "status": "Run"
    },
    {
      "pid": 3335,
      "name": "IntelligencePlatformComputeService",
      "status": "Run"
    },
    {
      "pid": 207,
      "name": "logd_helper",
      "status": "Unknown(0)"
    },
    {
      "pid": 1004,
      "name": "homed",
      "status": "Run"
    },
    {
      "pid": 24348,
      "name": "zsh",
      "status": "Run"
    },
    {
      "pid": 17338,
      "name": "colorsync.useragent",
      "status": "Run"
    },
    {
      "pid": 2986,
      "name": "AMPDevicesAgent",
      "status": "Run"
    },
    {
      "pid": 3741,
      "name": "fbahelperd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1014,
      "name": "FindMyWidgetPeople",
      "status": "Run"
    },
    {
      "pid": 35652,
      "name": "Safari",
      "status": "Run"
    },
    {
      "pid": 1942,
      "name": "Google Chrome Helper (GPU)",
      "status": "Run"
    },
    {
      "pid": 726,
      "name": "UARPUpdaterServiceUSBPD",
      "status": "Unknown(0)"
    },
    {
      "pid": 1172,
      "name": "AdobeIPCBroker",
      "status": "Run"
    },
    {
      "pid": 3473,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 389,
      "name": "cfprefsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 68302,
      "name": "git",
      "status": "Run"
    },
    {
      "pid": 49837,
      "name": "com.apple.WebKit.WebContent",
      "status": "Run"
    },
    {
      "pid": 35693,
      "name": "SimulatorTrampoline",
      "status": "Run"
    },
    {
      "pid": 72799,
      "name": "rustrover",
      "status": "Run"
    },
    {
      "pid": 3897,
      "name": "metrickitd",
      "status": "Run"
    },
    {
      "pid": 3701,
      "name": "mlhostd",
      "status": "Run"
    },
    {
      "pid": 3674,
      "name": "OSDUIHelper",
      "status": "Run"
    },
    {
      "pid": 3333,
      "name": "AppleMobileDeviceHelper",
      "status": "Run"
    },
    {
      "pid": 995,
      "name": "ScopedBookmarkAgent",
      "status": "Run"
    },
    {
      "pid": 900,
      "name": "bird",
      "status": "Run"
    },
    {
      "pid": 2146,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 925,
      "name": "imklaunchagent",
      "status": "Run"
    },
    {
      "pid": 1058,
      "name": "commerce",
      "status": "Run"
    },
    {
      "pid": 942,
      "name": "ViewBridgeAuxiliary",
      "status": "Run"
    },
    {
      "pid": 2389,
      "name": "SoftwareUpdateNotificationManager",
      "status": "Run"
    },
    {
      "pid": 862,
      "name": "GSSCred",
      "status": "Unknown(0)"
    },
    {
      "pid": 4407,
      "name": "intelligenceplatformd",
      "status": "Run"
    },
    {
      "pid": 49843,
      "name": "com.apple.WebKit.WebContent",
      "status": "Run"
    },
    {
      "pid": 9844,
      "name": "SoftwareUpdateSubscriber",
      "status": "Run"
    },
    {
      "pid": 3347,
      "name": "DockHelper",
      "status": "Run"
    },
    {
      "pid": 1099,
      "name": "RunCat",
      "status": "Run"
    },
    {
      "pid": 20022,
      "name": "lskdd",
      "status": "Unknown(0)"
    },
    {
      "pid": 883,
      "name": "com.apple.sbd",
      "status": "Run"
    },
    {
      "pid": 145,
      "name": "IOMFB_bics_daemon",
      "status": "Unknown(0)"
    },
    {
      "pid": 1028,
      "name": "coresymbolicationd",
      "status": "Unknown(0)"
    },
    {
      "pid": 348,
      "name": "nesessionmanager",
      "status": "Unknown(0)"
    },
    {
      "pid": 3027,
      "name": "VTEncoderXPCService",
      "status": "Run"
    },
    {
      "pid": 153,
      "name": "softwareupdated",
      "status": "Unknown(0)"
    },
    {
      "pid": 3465,
      "name": "WardaSynthesizer_arm64",
      "status": "Run"
    },
    {
      "pid": 678,
      "name": "ctkd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1053,
      "name": "com.apple.FaceTime.FTConversationService",
      "status": "Run"
    },
    {
      "pid": 219,
      "name": "runningboardd",
      "status": "Unknown(0)"
    },
    {
      "pid": 68354,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 72871,
      "name": "QuickLookUIService",
      "status": "Run"
    },
    {
      "pid": 31092,
      "name": "XprotectService",
      "status": "Run"
    },
    {
      "pid": 3237,
      "name": "naturallanguaged",
      "status": "Run"
    },
    {
      "pid": 31096,
      "name": "com.apple.appkit.xpc.openAndSavePanelService",
      "status": "Run"
    },
    {
      "pid": 3703,
      "name": "ASPCarryLog",
      "status": "Unknown(0)"
    },
    {
      "pid": 3047,
      "name": "secd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3705,
      "name": "backgroundassets.user",
      "status": "Run"
    },
    {
      "pid": 1687,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 921,
      "name": "CoreLocationAgent",
      "status": "Run"
    },
    {
      "pid": 9843,
      "name": "AccountSubscriber",
      "status": "Run"
    },
    {
      "pid": 911,
      "name": "trustd",
      "status": "Run"
    },
    {
      "pid": 25520,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 35655,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 2383,
      "name": "httpd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1290,
      "name": "SidecarRelay",
      "status": "Run"
    },
    {
      "pid": 18035,
      "name": "VTDecoderXPCService",
      "status": "Run"
    },
    {
      "pid": 1702,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 941,
      "name": "deleted",
      "status": "Run"
    },
    {
      "pid": 243,
      "name": "containermanagerd_system",
      "status": "Unknown(0)"
    },
    {
      "pid": 3019,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 9829,
      "name": "tzd",
      "status": "Unknown(0)"
    },
    {
      "pid": 20615,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 68012,
      "name": "Google Chrome Helper",
      "status": "Run"
    },
    {
      "pid": 1036,
      "name": "diagnosticextensionsd",
      "status": "Run"
    },
    {
      "pid": 2952,
      "name": "contentlinkingd",
      "status": "Run"
    },
    {
      "pid": 1032,
      "name": "financed",
      "status": "Run"
    },
    {
      "pid": 890,
      "name": "Finder",
      "status": "Run"
    },
    {
      "pid": 882,
      "name": "universalaccessd",
      "status": "Run"
    },
    {
      "pid": 1945,
      "name": "Google Chrome Helper",
      "status": "Run"
    },
    {
      "pid": 17388,
      "name": "webprivacyd",
      "status": "Run"
    },
    {
      "pid": 34660,
      "name": "zsh",
      "status": "Run"
    },
    {
      "pid": 330,
      "name": "trustd",
      "status": "Unknown(0)"
    },
    {
      "pid": 22191,
      "name": "Updater",
      "status": "Run"
    },
    {
      "pid": 1092,
      "name": "TextInputMenuAgent",
      "status": "Run"
    },
    {
      "pid": 17282,
      "name": "MTLCompilerService",
      "status": "Unknown(0)"
    },
    {
      "pid": 368,
      "name": "wifip2pd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1104,
      "name": "storekitagent",
      "status": "Run"
    },
    {
      "pid": 173,
      "name": "launchservicesd",
      "status": "Unknown(0)"
    },
    {
      "pid": 21635,
      "name": "VTDecoderXPCService",
      "status": "Run"
    },
    {
      "pid": 25035,
      "name": "Bitrix24 Helper",
      "status": "Run"
    },
    {
      "pid": 2983,
      "name": "promotedcontentd",
      "status": "Run"
    },
    {
      "pid": 144,
      "name": "powerd",
      "status": "Unknown(0)"
    },
    {
      "pid": 899,
      "name": "dmd",
      "status": "Run"
    },
    {
      "pid": 318,
      "name": "securityd_system",
      "status": "Unknown(0)"
    },
    {
      "pid": 985,
      "name": "CursorUIViewService",
      "status": "Run"
    },
    {
      "pid": 21639,
      "name": "BDisk",
      "status": "Run"
    },
    {
      "pid": 71837,
      "name": "login",
      "status": "Unknown(0)"
    },
    {
      "pid": 392,
      "name": "containermanagerd",
      "status": "Unknown(0)"
    },
    {
      "pid": 35656,
      "name": "com.apple.Safari.SearchHelper",
      "status": "Run"
    },
    {
      "pid": 196,
      "name": "AirPlayXPCHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 134,
      "name": "uninstalld",
      "status": "Unknown(0)"
    },
    {
      "pid": 902,
      "name": "nsurlsessiond",
      "status": "Run"
    },
    {
      "pid": 180,
      "name": "autofsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 68407,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 19718,
      "name": "Termius",
      "status": "Run"
    },
    {
      "pid": 9808,
      "name": "mmaintenanced",
      "status": "Unknown(0)"
    },
    {
      "pid": 5288,
      "name": "com.apple.CloudPhotosConfiguration",
      "status": "Run"
    },
    {
      "pid": 1003,
      "name": "followupd",
      "status": "Run"
    },
    {
      "pid": 1013,
      "name": "BatteriesAvocadoWidgetExtension",
      "status": "Run"
    },
    {
      "pid": 919,
      "name": "fileproviderd",
      "status": "Run"
    },
    {
      "pid": 51888,
      "name": "DataDetectorsSourceAccess",
      "status": "Unknown(0)"
    },
    {
      "pid": 72877,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 59067,
      "name": "Adobe Genuine Software Monitor Service",
      "status": "Run"
    },
    {
      "pid": 21633,
      "name": "Bitrix24 Helper",
      "status": "Run"
    },
    {
      "pid": 17856,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 18076,
      "name": "com.apple.DriverKit.AppleUserECM",
      "status": "Unknown(0)"
    },
    {
      "pid": 1050,
      "name": "ctkahp",
      "status": "Run"
    },
    {
      "pid": 2619,
      "name": "sbis-cef-helper",
      "status": "Run"
    },
    {
      "pid": 963,
      "name": "contactsd",
      "status": "Run"
    },
    {
      "pid": 718,
      "name": "wifianalyticsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 385,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 68350,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 17364,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 3544,
      "name": "ManagementTestSubscriber",
      "status": "Unknown(0)"
    },
    {
      "pid": 274,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 1077,
      "name": "LOGINserver",
      "status": "Run"
    },
    {
      "pid": 18074,
      "name": "com.apple.ifdreader",
      "status": "Unknown(0)"
    },
    {
      "pid": 3508,
      "name": "com.apple.accessibility.mediaaccessibilityd",
      "status": "Run"
    },
    {
      "pid": 19755,
      "name": "VisualizerService_x86",
      "status": "Run"
    },
    {
      "pid": 1113,
      "name": "adprivacyd",
      "status": "Run"
    },
    {
      "pid": 68346,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 17285,
      "name": "rapportd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1059,
      "name": "BiomeAgent",
      "status": "Run"
    },
    {
      "pid": 986,
      "name": "iCloudNotificationAgent",
      "status": "Run"
    },
    {
      "pid": 1370,
      "name": "VTDecoderXPCService",
      "status": "Run"
    },
    {
      "pid": 878,
      "name": "gamecontrollerd",
      "status": "Unknown(0)"
    },
    {
      "pid": 858,
      "name": "IOUserBluetoothSerialDriver",
      "status": "Unknown(0)"
    },
    {
      "pid": 870,
      "name": "cfprefsd",
      "status": "Run"
    },
    {
      "pid": 245,
      "name": "usbd",
      "status": "Unknown(0)"
    },
    {
      "pid": 20618,
      "name": "VTDecoderXPCService",
      "status": "Run"
    },
    {
      "pid": 301,
      "name": "mDNSResponderHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 3456,
      "name": "com.apple.SiriTTSService.TrialProxy",
      "status": "Run"
    },
    {
      "pid": 169,
      "name": "thermalmonitord",
      "status": "Unknown(0)"
    },
    {
      "pid": 1041,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 728,
      "name": "UARPUpdaterServiceHID",
      "status": "Unknown(0)"
    },
    {
      "pid": 1272,
      "name": "ANECompilerService",
      "status": "Unknown(0)"
    },
    {
      "pid": 35706,
      "name": "MDRemoteServiceSupport",
      "status": "Run"
    },
    {
      "pid": 3029,
      "name": "com.apple.SiriTTSService.TrialProxy",
      "status": "Run"
    },
    {
      "pid": 1155,
      "name": "findmydevice-user-agent",
      "status": "Run"
    },
    {
      "pid": 49912,
      "name": "CSExattrCryptoService",
      "status": "Run"
    },
    {
      "pid": 9850,
      "name": "sysdiagnosed",
      "status": "Unknown(0)"
    },
    {
      "pid": 2981,
      "name": "AssetCacheLocatorService",
      "status": "Unknown(0)"
    },
    {
      "pid": 1061,
      "name": "itunescloudd",
      "status": "Run"
    },
    {
      "pid": 1052,
      "name": "assistant_cdmd",
      "status": "Run"
    },
    {
      "pid": 1029,
      "name": "AssetCacheLocatorService",
      "status": "Run"
    },
    {
      "pid": 913,
      "name": "filecoordinationd",
      "status": "Unknown(0)"
    },
    {
      "pid": 341,
      "name": "PerfPowerTelemetryClientRegistrationService",
      "status": "Unknown(0)"
    },
    {
      "pid": 331,
      "name": "com.apple.CodeSigningHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 178,
      "name": "locationd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3489,
      "name": "PlugInLibraryService",
      "status": "Run"
    },
    {
      "pid": 3896,
      "name": "extensionkitservice",
      "status": "Run"
    },
    {
      "pid": 1018,
      "name": "MTLAssetUpgraderD",
      "status": "Run"
    },
    {
      "pid": 2985,
      "name": "gamecontrolleragentd",
      "status": "Run"
    },
    {
      "pid": 886,
      "name": "pboard",
      "status": "Run"
    },
    {
      "pid": 3471,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 5930,
      "name": "dprivacyd",
      "status": "Unknown(0)"
    },
    {
      "pid": 21638,
      "name": "Bitrix24 Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 3101,
      "name": "mdworker",
      "status": "Run"
    },
    {
      "pid": 3536,
      "name": "ScreenSharingSubscriber",
      "status": "Unknown(0)"
    },
    {
      "pid": 3899,
      "name": "powerdatad",
      "status": "Unknown(0)"
    },
    {
      "pid": 4019,
      "name": "PerfPowerTelemetryClientRegistrationService",
      "status": "Unknown(0)"
    },
    {
      "pid": 2628,
      "name": "ChromeNmhTransport",
      "status": "Run"
    },
    {
      "pid": 1118,
      "name": "TextInputSwitcher",
      "status": "Run"
    },
    {
      "pid": 1007,
      "name": "CMFSyncAgent",
      "status": "Run"
    },
    {
      "pid": 35698,
      "name": "com.apple.appkit.xpc.openAndSavePanelService",
      "status": "Run"
    },
    {
      "pid": 2957,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 994,
      "name": "intelligentroutingd",
      "status": "Run"
    },
    {
      "pid": 966,
      "name": "fontworker",
      "status": "Run"
    },
    {
      "pid": 34528,
      "name": "WhatsApp",
      "status": "Run"
    },
    {
      "pid": 28406,
      "name": "jcef Helper",
      "status": "Run"
    },
    {
      "pid": 24939,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 35672,
      "name": "diskimagesiod",
      "status": "Unknown(0)"
    },
    {
      "pid": 3520,
      "name": "remotemanagementd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1060,
      "name": "mediaremoteagent",
      "status": "Run"
    },
    {
      "pid": 136,
      "name": "mediaremoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 19724,
      "name": "Termius Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 17347,
      "name": "ioupsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3565,
      "name": "SpeechSynthesisServerXPC",
      "status": "Run"
    },
    {
      "pid": 3533,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 68380,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 72876,
      "name": "CSExattrCryptoService",
      "status": "Run"
    },
    {
      "pid": 54214,
      "name": "com.apple.AppStoreDaemon.StorePrivilegedTaskService",
      "status": "Unknown(0)"
    },
    {
      "pid": 19723,
      "name": "Termius Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 976,
      "name": "knowledge-agent",
      "status": "Run"
    },
    {
      "pid": 17567,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 967,
      "name": "com.apple.dock.extra",
      "status": "Run"
    },
    {
      "pid": 349,
      "name": "com.apple.audio.Core-Audio-Driver-Service.helper",
      "status": "Unknown(0)"
    },
    {
      "pid": 199,
      "name": "tccd",
      "status": "Unknown(0)"
    },
    {
      "pid": 807,
      "name": "gamepolicyd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3474,
      "name": "voicebankingd",
      "status": "Run"
    },
    {
      "pid": 21637,
      "name": "Bitrix24 Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 3500,
      "name": "cfprefsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 2502,
      "name": "sbis-cef-helper",
      "status": "Run"
    },
    {
      "pid": 78667,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 3035,
      "name": "AMPLibraryAgent",
      "status": "Run"
    },
    {
      "pid": 3046,
      "name": "trustd",
      "status": "Unknown(0)"
    },
    {
      "pid": 403,
      "name": "containermanagerd",
      "status": "Unknown(0)"
    },
    {
      "pid": 367,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 1048,
      "name": "SubmitDiagInfo",
      "status": "Unknown(0)"
    },
    {
      "pid": 1373,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 3240,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 67675,
      "name": "java",
      "status": "Run"
    },
    {
      "pid": 2058,
      "name": "online-auth-agent",
      "status": "Unknown(0)"
    },
    {
      "pid": 1176,
      "name": "mdbulkimport",
      "status": "Run"
    },
    {
      "pid": 13881,
      "name": "VTDecoderXPCService",
      "status": "Run"
    },
    {
      "pid": 1054,
      "name": "sirittsd",
      "status": "Run"
    },
    {
      "pid": 1037,
      "name": "maild",
      "status": "Run"
    },
    {
      "pid": 868,
      "name": "distnoted",
      "status": "Run"
    },
    {
      "pid": 3498,
      "name": "nsattributedstringagent",
      "status": "Unknown(0)"
    },
    {
      "pid": 350,
      "name": "com.apple.audio.DriverHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 1114,
      "name": "weatherd",
      "status": "Run"
    },
    {
      "pid": 277,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 957,
      "name": "FinderExtension",
      "status": "Run"
    },
    {
      "pid": 238,
      "name": "cryptexd",
      "status": "Unknown(0)"
    },
    {
      "pid": 130,
      "name": "logd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1012,
      "name": "FamilyControlsAgent",
      "status": "Run"
    },
    {
      "pid": 81405,
      "name": "MessagesBlastDoorService",
      "status": "Run"
    },
    {
      "pid": 352,
      "name": "oahd",
      "status": "Unknown(0)"
    },
    {
      "pid": 20144,
      "name": "USBAgent",
      "status": "Run"
    },
    {
      "pid": 973,
      "name": "NotificationCenter",
      "status": "Run"
    },
    {
      "pid": 735,
      "name": "secinitd",
      "status": "Unknown(0)"
    },
    {
      "pid": 68413,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 205,
      "name": "aslmanager",
      "status": "Unknown(0)"
    },
    {
      "pid": 997,
      "name": "WorldClockWidget",
      "status": "Run"
    },
    {
      "pid": 328,
      "name": "sysextd",
      "status": "Unknown(0)"
    },
    {
      "pid": 2339,
      "name": "progressd",
      "status": "Run"
    },
    {
      "pid": 212,
      "name": "coreservicesd",
      "status": "Unknown(0)"
    },
    {
      "pid": 281,
      "name": "authd",
      "status": "Unknown(0)"
    },
    {
      "pid": 49856,
      "name": "com.apple.WebKit.WebContent",
      "status": "Run"
    },
    {
      "pid": 16653,
      "name": "ServiceExtension",
      "status": "Run"
    },
    {
      "pid": 3025,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 181,
      "name": "dasd",
      "status": "Unknown(0)"
    },
    {
      "pid": 35665,
      "name": "remotepairingd",
      "status": "Run"
    },
    {
      "pid": 887,
      "name": "Dock",
      "status": "Run"
    },
    {
      "pid": 197,
      "name": "com.apple.cmio.registerassistantservice",
      "status": "Unknown(0)"
    },
    {
      "pid": 19306,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 64443,
      "name": "AGSService",
      "status": "Unknown(0)"
    },
    {
      "pid": 1042,
      "name": "ContinuityCaptureAgent",
      "status": "Run"
    },
    {
      "pid": 280,
      "name": "secinitd",
      "status": "Unknown(0)"
    },
    {
      "pid": 194,
      "name": "sandboxd",
      "status": "Unknown(0)"
    },
    {
      "pid": 135,
      "name": "fseventsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 19721,
      "name": "Termius Helper",
      "status": "Run"
    },
    {
      "pid": 3015,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 36826,
      "name": "com.apple.WebKit.WebContent",
      "status": "Run"
    },
    {
      "pid": 1031,
      "name": "lockdownmoded",
      "status": "Run"
    },
    {
      "pid": 897,
      "name": "QuickLookUIService",
      "status": "Run"
    },
    {
      "pid": 43115,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 3519,
      "name": "DistributionHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 35705,
      "name": "MDRemoteServiceSupport",
      "status": "Run"
    },
    {
      "pid": 4066,
      "name": "knowledgeconstructiond",
      "status": "Run"
    },
    {
      "pid": 944,
      "name": "finder_ext",
      "status": "Run"
    },
    {
      "pid": 796,
      "name": "captiveagent",
      "status": "Unknown(0)"
    },
    {
      "pid": 3450,
      "name": "MacinTalkAUSP",
      "status": "Run"
    },
    {
      "pid": 901,
      "name": "APFSUserAgent",
      "status": "Run"
    },
    {
      "pid": 4026,
      "name": "AuthenticationServicesAgent",
      "status": "Run"
    },
    {
      "pid": 3018,
      "name": "mlruntimed",
      "status": "Run"
    },
    {
      "pid": 329,
      "name": "mobileactivationd",
      "status": "Unknown(0)"
    },
    {
      "pid": 294,
      "name": "findmydeviced",
      "status": "Unknown(0)"
    },
    {
      "pid": 57168,
      "name": "php-fpm",
      "status": "Run"
    },
    {
      "pid": 22006,
      "name": "QuickLookUIService",
      "status": "Run"
    },
    {
      "pid": 25038,
      "name": "VTEncoderXPCService",
      "status": "Run"
    },
    {
      "pid": 3488,
      "name": "SafariBookmarksSyncAgent",
      "status": "Run"
    },
    {
      "pid": 172,
      "name": "httpd",
      "status": "Unknown(0)"
    },
    {
      "pid": 21995,
      "name": "Snagit 2022",
      "status": "Run"
    },
    {
      "pid": 9835,
      "name": "SecuritySubscriber",
      "status": "Run"
    },
    {
      "pid": 4478,
      "name": "XProtect",
      "status": "Run"
    },
    {
      "pid": 3274,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 1377,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 971,
      "name": "ctkd",
      "status": "Run"
    },
    {
      "pid": 1271,
      "name": "installd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3271,
      "name": "Terminal",
      "status": "Run"
    },
    {
      "pid": 895,
      "name": "secd",
      "status": "Run"
    },
    {
      "pid": 52665,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 68176,
      "name": "EAUpdaterService",
      "status": "Unknown(0)"
    },
    {
      "pid": 51872,
      "name": "mbuseragent",
      "status": "Run"
    },
    {
      "pid": 3531,
      "name": "XprotectService",
      "status": "Unknown(0)"
    },
    {
      "pid": 2417,
      "name": "VTDecoderXPCService",
      "status": "Run"
    },
    {
      "pid": 1098,
      "name": "amsengagementd",
      "status": "Run"
    },
    {
      "pid": 3740,
      "name": "betaenrollmentd",
      "status": "Run"
    },
    {
      "pid": 1001,
      "name": "cdpd",
      "status": "Run"
    },
    {
      "pid": 951,
      "name": "callservicesd",
      "status": "Run"
    },
    {
      "pid": 3651,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 3563,
      "name": "QLPreviewGenerationExtension",
      "status": "Run"
    },
    {
      "pid": 9845,
      "name": "ManagementTestSubscriber",
      "status": "Run"
    },
    {
      "pid": 1091,
      "name": "diagnostics_agent",
      "status": "Run"
    },
    {
      "pid": 894,
      "name": "lsd",
      "status": "Run"
    },
    {
      "pid": 68406,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 2988,
      "name": "SetStoreUpdateService",
      "status": "Run"
    },
    {
      "pid": 9886,
      "name": "media-indexer",
      "status": "Run"
    },
    {
      "pid": 3702,
      "name": "homeenergyd",
      "status": "Run"
    },
    {
      "pid": 2418,
      "name": "VTDecoderXPCService",
      "status": "Run"
    },
    {
      "pid": 1275,
      "name": "UniversalControl",
      "status": "Run"
    },
    {
      "pid": 49839,
      "name": "com.apple.WebKit.Networking",
      "status": "Run"
    },
    {
      "pid": 22139,
      "name": "com.apple.photos.ImageConversionService",
      "status": "Run"
    },
    {
      "pid": 3818,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 22190,
      "name": "Autoupdate",
      "status": "Run"
    },
    {
      "pid": 3031,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 3044,
      "name": "lsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1072,
      "name": "App Cleaner Helper",
      "status": "Run"
    },
    {
      "pid": 937,
      "name": "donotdisturbd",
      "status": "Run"
    },
    {
      "pid": 49840,
      "name": "com.apple.WebKit.GPU",
      "status": "Run"
    },
    {
      "pid": 326,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 68390,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 17314,
      "name": "coreautha",
      "status": "Run"
    },
    {
      "pid": 930,
      "name": "networkserviceproxy",
      "status": "Run"
    },
    {
      "pid": 35691,
      "name": "SimLaunchHost.arm64",
      "status": "Run"
    },
    {
      "pid": 778,
      "name": "containermanagerd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3461,
      "name": "triald_system",
      "status": "Unknown(0)"
    },
    {
      "pid": 3476,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 916,
      "name": "automountd",
      "status": "Unknown(0)"
    },
    {
      "pid": 321,
      "name": "WirelessRadioManagerd",
      "status": "Unknown(0)"
    },
    {
      "pid": 174,
      "name": "timed",
      "status": "Unknown(0)"
    },
    {
      "pid": 3532,
      "name": "ReportCrash",
      "status": "Unknown(0)"
    },
    {
      "pid": 1102,
      "name": "deleted_helper",
      "status": "Unknown(0)"
    },
    {
      "pid": 917,
      "name": "sharedfilelistd",
      "status": "Run"
    },
    {
      "pid": 3537,
      "name": "CategoriesService",
      "status": "Run"
    },
    {
      "pid": 317,
      "name": "syspolicyd",
      "status": "Unknown(0)"
    },
    {
      "pid": 247,
      "name": "csnameddatad",
      "status": "Unknown(0)"
    },
    {
      "pid": 264,
      "name": "trustdFileHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 18094,
      "name": "UVFSService",
      "status": "Unknown(0)"
    },
    {
      "pid": 35653,
      "name": "com.apple.Safari.History",
      "status": "Run"
    },
    {
      "pid": 3546,
      "name": "ASConfigurationSubscriber",
      "status": "Unknown(0)"
    },
    {
      "pid": 14946,
      "name": "navd",
      "status": "Run"
    },
    {
      "pid": 3022,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 315,
      "name": "colorsync.displayservices",
      "status": "Unknown(0)"
    },
    {
      "pid": 184,
      "name": "AppleCredentialManagerDaemon",
      "status": "Unknown(0)"
    },
    {
      "pid": 43148,
      "name": "com.apple.SafariServices",
      "status": "Run"
    },
    {
      "pid": 18228,
      "name": "jcef Helper (GPU)",
      "status": "Run"
    },
    {
      "pid": 18093,
      "name": "userfsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 1051,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 141,
      "name": "accessoryupdaterd",
      "status": "Unknown(0)"
    },
    {
      "pid": 190,
      "name": "usermanagerd",
      "status": "Unknown(0)"
    },
    {
      "pid": 4027,
      "name": "keychainsharingmessagingd",
      "status": "Run"
    },
    {
      "pid": 970,
      "name": "transparencyd",
      "status": "Run"
    },
    {
      "pid": 187,
      "name": "logind",
      "status": "Unknown(0)"
    },
    {
      "pid": 35667,
      "name": "com.apple.CoreSimulator.CoreSimulatorService",
      "status": "Run"
    },
    {
      "pid": 990,
      "name": "replayd",
      "status": "Run"
    },
    {
      "pid": 3049,
      "name": "pycharm",
      "status": "Run"
    },
    {
      "pid": 72870,
      "name": "com.apple.appkit.xpc.openAndSavePanelService",
      "status": "Run"
    },
    {
      "pid": 1056,
      "name": "triald",
      "status": "Run"
    },
    {
      "pid": 1005,
      "name": "AppSSODaemon",
      "status": "Unknown(0)"
    },
    {
      "pid": 150,
      "name": "remoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 235,
      "name": "airportd",
      "status": "Unknown(0)"
    },
    {
      "pid": 975,
      "name": "localizationswitcherd",
      "status": "Run"
    },
    {
      "pid": 65927,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 3020,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 3469,
      "name": "MailShortcutsExtension",
      "status": "Run"
    },
    {
      "pid": 3024,
      "name": "Google Chrome Helper (Plugin)",
      "status": "Run"
    },
    {
      "pid": 67648,
      "name": "fsnotifier",
      "status": "Run"
    },
    {
      "pid": 1123,
      "name": "Creative Cloud Content Manager.node",
      "status": "Run"
    },
    {
      "pid": 18078,
      "name": "usbsmartcardreaderd",
      "status": "Unknown(0)"
    },
    {
      "pid": 40659,
      "name": "VTEncoderXPCService",
      "status": "Run"
    },
    {
      "pid": 1291,
      "name": "installcoordinationd",
      "status": "Unknown(0)"
    },
    {
      "pid": 175,
      "name": "usbmuxd",
      "status": "Unknown(0)"
    },
    {
      "pid": 5298,
      "name": "MTLCompilerService",
      "status": "Unknown(0)"
    },
    {
      "pid": 56268,
      "name": "LookupViewService",
      "status": "Run"
    },
    {
      "pid": 993,
      "name": "heard",
      "status": "Run"
    },
    {
      "pid": 3543,
      "name": "SoftwareUpdateSubscriber",
      "status": "Unknown(0)"
    },
    {
      "pid": 19766,
      "name": "passd",
      "status": "Run"
    },
    {
      "pid": 65926,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 146,
      "name": "biomed",
      "status": "Unknown(0)"
    },
    {
      "pid": 143,
      "name": "endpointsecurityd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3718,
      "name": "ospredictiond",
      "status": "Unknown(0)"
    },
    {
      "pid": 75571,
      "name": "cargo",
      "status": "Run"
    },
    {
      "pid": 35664,
      "name": "com.apple.WebKit.GPU",
      "status": "Run"
    },
    {
      "pid": 3490,
      "name": "CrashReporterSupportHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 1166,
      "name": "osanalyticshelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 826,
      "name": "com.apple.ColorSyncXPCAgent",
      "status": "Unknown(0)"
    },
    {
      "pid": 18097,
      "name": "fskitd",
      "status": "Unknown(0)"
    },
    {
      "pid": 727,
      "name": "UARPUpdaterServiceLegacyAudio",
      "status": "Unknown(0)"
    },
    {
      "pid": 3704,
      "name": "liquiddetectiond",
      "status": "Unknown(0)"
    },
    {
      "pid": 1285,
      "name": "system_installd",
      "status": "Unknown(0)"
    },
    {
      "pid": 45060,
      "name": "TelemetryDiskChecker",
      "status": "Run"
    },
    {
      "pid": 3534,
      "name": "SecuritySubscriber",
      "status": "Unknown(0)"
    },
    {
      "pid": 3236,
      "name": "AppleSpell",
      "status": "Run"
    },
    {
      "pid": 45049,
      "name": "avatarsd",
      "status": "Run"
    },
    {
      "pid": 3157,
      "name": "NETserver",
      "status": "Run"
    },
    {
      "pid": 49991,
      "name": "Google Chrome Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 68411,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 1055,
      "name": "AudioComponentRegistrar",
      "status": "Run"
    },
    {
      "pid": 984,
      "name": "extensionkitservice",
      "status": "Run"
    },
    {
      "pid": 45048,
      "name": "contactsdonationagent",
      "status": "Run"
    },
    {
      "pid": 22123,
      "name": "nbagent",
      "status": "Run"
    },
    {
      "pid": 9810,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 3272,
      "name": "AMPArtworkAgent",
      "status": "Run"
    },
    {
      "pid": 3000,
      "name": "misagent",
      "status": "Unknown(0)"
    },
    {
      "pid": 1474,
      "name": "USBserver",
      "status": "Run"
    },
    {
      "pid": 3238,
      "name": "keyboardservicesd",
      "status": "Run"
    },
    {
      "pid": 947,
      "name": "swcd",
      "status": "Run"
    },
    {
      "pid": 898,
      "name": "csnameddatad",
      "status": "Run"
    },
    {
      "pid": 357,
      "name": "audioanalyticsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 932,
      "name": "accountsd",
      "status": "Run"
    },
    {
      "pid": 1296,
      "name": "MTLCompilerService",
      "status": "Unknown(0)"
    },
    {
      "pid": 912,
      "name": "WallpaperAgent",
      "status": "Run"
    },
    {
      "pid": 721,
      "name": "systemstats",
      "status": "Unknown(0)"
    },
    {
      "pid": 935,
      "name": "idleassetsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 244,
      "name": "accessoryd",
      "status": "Unknown(0)"
    },
    {
      "pid": 65682,
      "name": "com.apple.audio.SandboxHelper",
      "status": "Run"
    },
    {
      "pid": 1033,
      "name": "corespeechd",
      "status": "Run"
    },
    {
      "pid": 165,
      "name": "coreduetd",
      "status": "Unknown(0)"
    },
    {
      "pid": 18075,
      "name": "com.apple.DriverKit.AppleUserECM",
      "status": "Unknown(0)"
    },
    {
      "pid": 20606,
      "name": "com.apple.ColorSyncXPCAgent",
      "status": "Run"
    },
    {
      "pid": 20065,
      "name": "GroupSessionService",
      "status": "Run"
    },
    {
      "pid": 3034,
      "name": "mdbulkimport",
      "status": "Unknown(0)"
    },
    {
      "pid": 952,
      "name": "ACCFinderSync",
      "status": "Run"
    },
    {
      "pid": 142,
      "name": "configd",
      "status": "Unknown(0)"
    },
    {
      "pid": 21636,
      "name": "Bitrix24 Helper (Renderer)",
      "status": "Run"
    },
    {
      "pid": 1009,
      "name": "TrustedPeersHelper",
      "status": "Run"
    },
    {
      "pid": 17413,
      "name": "com.apple.tonelibraryd",
      "status": "Run"
    },
    {
      "pid": 3504,
      "name": "com.apple.accessibility.mediaaccessibilityd",
      "status": "Unknown(0)"
    },
    {
      "pid": 2979,
      "name": "com.apple.hiservices-xpcservice",
      "status": "Unknown(0)"
    },
    {
      "pid": 1110,
      "name": "PlugInLibraryService",
      "status": "Run"
    },
    {
      "pid": 206,
      "name": "lsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 3222,
      "name": "USBAppControl",
      "status": "Run"
    },
    {
      "pid": 961,
      "name": "useractivityd",
      "status": "Run"
    },
    {
      "pid": 351,
      "name": "kernelmanager_helper",
      "status": "Unknown(0)"
    },
    {
      "pid": 991,
      "name": "sociallayerd",
      "status": "Run"
    },
    {
      "pid": 306,
      "name": "TeamViewer_Service",
      "status": "Unknown(0)"
    },
    {
      "pid": 4463,
      "name": "mapssyncd",
      "status": "Run"
    },
    {
      "pid": 242,
      "name": "apfsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 22348,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 139,
      "name": "systemstats",
      "status": "Unknown(0)"
    },
    {
      "pid": 198,
      "name": "WindowServer",
      "status": "Unknown(0)"
    },
    {
      "pid": 394,
      "name": "distnoted",
      "status": "Unknown(0)"
    },
    {
      "pid": 908,
      "name": "lockoutagent",
      "status": "Run"
    },
    {
      "pid": 1045,
      "name": "parsecd",
      "status": "Run"
    },
    {
      "pid": 162,
      "name": "diskarbitrationd",
      "status": "Unknown(0)"
    },
    {
      "pid": 251,
      "name": "biometrickitd",
      "status": "Unknown(0)"
    },
    {
      "pid": 21632,
      "name": "Bitrix24 Helper",
      "status": "Run"
    },
    {
      "pid": 35658,
      "name": "com.apple.WebKit.Networking",
      "status": "Run"
    },
    {
      "pid": 382,
      "name": "softwareupdated",
      "status": "Unknown(0)"
    },
    {
      "pid": 68303,
      "name": "ssh",
      "status": "Run"
    },
    {
      "pid": 3203,
      "name": "WorkflowAppControl",
      "status": "Run"
    },
    {
      "pid": 988,
      "name": "WeatherWidget",
      "status": "Run"
    },
    {
      "pid": 17148,
      "name": "PowerChime",
      "status": "Run"
    },
    {
      "pid": 68410,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 19719,
      "name": "Termius Helper (GPU)",
      "status": "Run"
    },
    {
      "pid": 962,
      "name": "ManagedSettingsAgent",
      "status": "Run"
    },
    {
      "pid": 4398,
      "name": "PerfPowerTelemetryClientRegistrationService",
      "status": "Run"
    },
    {
      "pid": 954,
      "name": "syncdefaultsd",
      "status": "Run"
    },
    {
      "pid": 9837,
      "name": "ScreenSharingSubscriber",
      "status": "Run"
    },
    {
      "pid": 34702,
      "name": "Calendar",
      "status": "Run"
    },
    {
      "pid": 3021,
      "name": "Google Chrome Helper",
      "status": "Run"
    },
    {
      "pid": 972,
      "name": "coreauthd",
      "status": "Run"
    },
    {
      "pid": 948,
      "name": "linkd",
      "status": "Run"
    },
    {
      "pid": 171,
      "name": "apsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 889,
      "name": "SystemUIServer",
      "status": "Run"
    },
    {
      "pid": 877,
      "name": "UserEventAgent",
      "status": "Run"
    },
    {
      "pid": 334,
      "name": "appleh13camerad",
      "status": "Unknown(0)"
    },
    {
      "pid": 67610,
      "name": "com.apple.WebKit.WebContent",
      "status": "Run"
    },
    {
      "pid": 68360,
      "name": "mdworker_shared",
      "status": "Run"
    },
    {
      "pid": 3048,
      "name": "PlugInLibraryService",
      "status": "Run"
    },
    {
      "pid": 1292,
      "name": "com.apple.AppStoreDaemon.StorePrivilegedODRService",
      "status": "Unknown(0)"
    },
    {
      "pid": 3819,
      "name": "SSTP Connect",
      "status": "Run"
    },
    {
      "pid": 361,
      "name": "com.apple.AccountPolicyHelper",
      "status": "Unknown(0)"
    },
    {
      "pid": 35661,
      "name": "com.apple.Safari.SandboxBroker",
      "status": "Run"
    },
    {
      "pid": 31091,
      "name": "PlugInLibraryService",
      "status": "Run"
    },
    {
      "pid": 3348,
      "name": "fudHelperAgent",
      "status": "Run"
    },
    {
      "pid": 50167,
      "name": "AirPort Base Station Agent",
      "status": "Run"
    },
    {
      "pid": 1076,
      "name": "icdd",
      "status": "Run"
    },
    {
      "pid": 325,
      "name": "findmybeaconingd",
      "status": "Unknown(0)"
    },
    {
      "pid": 81408,
      "name": "MessagesBlastDoorService",
      "status": "Run"
    },
    {
      "pid": 927,
      "name": "ScreenTimeAgent",
      "status": "Run"
    },
    {
      "pid": 737,
      "name": "cfprefsd",
      "status": "Unknown(0)"
    },
    {
      "pid": 19753,
      "name": "Music",
      "status": "Run"
    },
    {
      "pid": 3816,
      "name": "appinstalld",
      "status": "Unknown(0)"
    },
    {
      "pid": 2409,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 1043,
      "name": "spindump",
      "status": "Unknown(0)"
    },
    {
      "pid": 905,
      "name": "secinitd",
      "status": "Run"
    },
    {
      "pid": 17283,
      "name": "MTLCompilerService",
      "status": "Unknown(0)"
    },
    {
      "pid": 35663,
      "name": "SafariNotificationAgent",
      "status": "Run"
    },
    {
      "pid": 968,
      "name": "PAH_Extension",
      "status": "Run"
    },
    {
      "pid": 35681,
      "name": "diskimagesiod",
      "status": "Unknown(0)"
    },
    {
      "pid": 3495,
      "name": "peopled",
      "status": "Run"
    },
    {
      "pid": 71839,
      "name": "zsh",
      "status": "Run"
    },
    {
      "pid": 2059,
      "name": "sbis3plugin",
      "status": "Run"
    },
    {
      "pid": 922,
      "name": "calaccessd",
      "status": "Run"
    },
    {
      "pid": 888,
      "name": "ControlCenter",
      "status": "Run"
    },
    {
      "pid": 36005,
      "name": "MTLCompilerService",
      "status": "Run"
    },
    {
      "pid": 734,
      "name": "com.apple.geod",
      "status": "Unknown(0)"
    },
    {
      "pid": 1046,
      "name": "QuickLookSatellite",
      "status": "Run"
    },
    {
      "pid": 17393,
      "name": "com.apple.Safari.SafeBrowsing.Service",
      "status": "Run"
    }
  ],
  "kernel_version": "macos",
  "execution_time": "1.19"
}
```
</details>