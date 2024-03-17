/*
 Generated by typeshare 1.7.0
*/

/** This is a generalized version of the state for all devices */
export interface SoundcoreDeviceState {
  featureFlags: BitFlags<SoundcoreFeatureFlags>;
  battery: Battery;
  soundMode: SoundMode;
  serial?: SerialNumber;
  fw?: FirmwareVer;
  drcFw?: FirmwareVer;
  hostDevice?: number;
  twsStatus?: TwsStatus;
  buttonModel?: ButtonModel;
  sideTone?: SideTone;
  hearidEqPreset?: number;
  wearDetection?: WearDetection;
  hearId?: HearID;
  ageRange?: AgeRange;
}

export interface BluetoothAdrr {
  address: [number, number, number, number, number, number];
}

export interface BLEDeviceDescriptor {
  addr: BluetoothAdrr;
  name: string;
}

export enum SupportedModels {
  A3027 = 'A3027',
  A3028 = 'A3028',
  A3029 = 'A3029',
  A3040 = 'A3040',
  A3930 = 'A3930',
  A3931 = 'A3931',
  A3935 = 'A3935',
  A3951 = 'A3951'
}

/** A discovered BLE device. The DiscoveredDevice can be upgraded to a SoundcoreBLEDevice. */
export interface DiscoveredDevice {
  /** The BLE device descriptor. */
  descriptor: BLEDeviceDescriptor;
  /** The model of the device, resolved using the device's advertised name. */
  model?: SupportedModels;
}

export interface DeviceInfo {
  left_fw: string;
  right_fw: string;
  sn: string;
}

export interface BatteryLevel {
  left: number;
  right: number;
}

export interface BatteryCharging {
  left: boolean;
  right: boolean;
}

export interface ANCProfile {
  option: number;
  anc_option: number;
  transparency_option: number;
  anc_custom: number;
}

export interface DeviceStatus {
  host_device: number;
  tws_status: boolean;
  battery_level: BatteryLevel;
  battery_charging: BatteryCharging;
  anc_status: ANCProfile;
  side_tone_enabled: boolean;
  wear_detection_enabled: boolean;
  touch_tone_enabled: boolean;
  left_eq: EQWave;
  right_eq: EQWave;
  hearid_enabled: boolean;
  left_hearid: EQWave;
  right_hearid: EQWave;
  left_hearid_customdata: EQWave;
  right_hearid_customdata: EQWave;
}

export type BLEAdapterEvent =
  | { kind: 'deviceConnected'; value: BluetoothAdrr }
  | { kind: 'deviceDisconnected'; value: BluetoothAdrr };

export enum SoundcoreFeatureFlags {
  SOUND_MODE = 'SOUND_MODE',
  ANC_MODE = 'ANC_MODE',
  TRANS_MODE = 'TRANS_MODE',
  CUSTOM_ANC = 'CUSTOM_ANC',
  EQ = 'EQ',
  STEREO_EQ = 'STEREO_EQ',
  DRC = 'DRC',
  HEARID = 'HEARID',
  WEAR_DETECTION = 'WEAR_DETECTION',
  CUSTOM_BUTTONS = 'CUSTOM_BUTTONS',
  TOUCH_TONE = 'TOUCH_TONE',
  GAME_MODE = 'GAME_MODE',
  AUTO_POWER_OFF_ON = 'AUTO_POWER_OFF_ON',
  InEarBeep = 'IN_EAR_BEEP',
  PromptLang = 'PROMPT_LANG',
  HearingProtect = 'HEARING_PROTECT',
  AmbientSoundNotice = 'AMBIENT_SOUND_NOTICE',
  PowerOnBatteryNotice = 'POWER_ON_BATTERY_NOTICE',
  SupportTwoCnn = 'SUPPORT_TWO_CNN',
  MultipleDeviceList = 'MULTIPLE_DEVICE_LIST'
}
