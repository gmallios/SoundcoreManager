/*
 Generated by typeshare 1.8.0
*/

export type HearIDType = number;

export type HearIDMusicType = number;

/**
 *
 * "Toggle" types, booleans that represent a toggleable feature and are parsed using the bool_parser.
 *
 */
export type GameMode = boolean;

export type BassUp = boolean;

export type LDAC = boolean;

export type InEarBeep = boolean;

export type SupportTwoCnn = boolean;

export type ThreeDimensionalEffect = boolean;

export type SideTone = boolean;

export type PowerOnBatteryNotice = boolean;

export type TwsStatus = boolean;

export type WearDetection = boolean;

export type TouchTone = boolean;

export type AgeRange = number;

export type AmbientSoundNotice = boolean;

export type Battery =
  | { type: 'single'; value: SingleBattery }
  | { type: 'dual'; value: DualBattery };

export enum CurrentSoundMode {
  ANC = 'ANC',
  Transparency = 'TRANSPARENCY',
  Normal = 'NORMAL'
}

export interface SoundMode {
  current: CurrentSoundMode;
  ancMode: ANCMode;
  transMode: TransparencyMode;
  customAnc: CustomANCValue;
  customTrans?: CustomTransparencyValue;
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

export interface SerialNumber {
  value: string;
  model?: SupportedModels;
}

export interface FirmwareVer {
  major: number;
  minor: number;
}

export type ButtonModel =
  | { type: 'a3909'; value: A3909ButtonModel }
  | { type: 'a3040'; value: A3040ButtonModel };

export type HearID = { type: 'BASE'; value: BaseHearID } | { type: 'CUSTOM'; value: CustomHearID };

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

/** A discovered BLE device. The DiscoveredDevice can be upgraded to a SoundcoreBLEDevice. */
export interface DiscoveredDevice {
  /** The BLE device descriptor. */
  descriptor: BLEDeviceDescriptor;
  /** The model of the device, resolved using the device's advertised name. */
  model?: SupportedModels;
}

export enum Action {
  VolumeUp = 'VOLUME_UP',
  VolumeDown = 'VOLUME_DOWN',
  PreviousSong = 'PREVIOUS_SONG',
  NextSong = 'NEXT_SONG',
  AmbientSound = 'AMBIENT_SOUND',
  VoiceAssistant = 'VOICE_ASSISTANT',
  PlayPause = 'PLAY_PAUSE',
  BassUpToggle = 'BASS_UP_TOGGLE',
  Null = 'NULL',
  ControlThreeDimensionalEffect = 'CONTROL_THREE_DIMENSIONAL_EFFECT'
}

export interface A3040ButtonModel {
  single_click: Action;
  double_click: Action;
}

export interface SingleBattery {
  charging: boolean;
  level: number;
}

export interface DualBattery {
  left: SingleBattery;
  right: SingleBattery;
}

export interface MonoEQ {
  /** * The values that we store are what is
   * received/sent and clamped within the range of 0..=240 */
  values: number[];
}

export interface StereoEQ {
  left: MonoEQ;
  right: MonoEQ;
}

export interface StereoEQConfiguration {
  eq: StereoEQ;
  profile: EQProfile;
}

export interface MonoEQConfiguration {
  eq: MonoEQ;
  profile: EQProfile;
}

export interface BaseHearID {
  enabled: boolean;
  values: StereoEQ;
  time: number;
}

export interface CustomHearID {
  base: BaseHearID;
  hearid_type: HearIDType;
  hearid_music_type: HearIDMusicType;
  custom_values?: StereoEQ;
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

export type EQConfiguration =
  | { type: 'stereo'; value: StereoEQConfiguration }
  | { type: 'mono'; value: MonoEQConfiguration };

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
  IN_EAR_BEEP = 'IN_EAR_BEEP',
  LANG_PROMPT = 'LANG_PROMPT',
  HEARING_PROTECTION = 'HEARING_PROTECTION',
  AMBIENT_SOUND_NOTICE = 'AMBIENT_SOUND_NOTICE',
  POWER_ON_BATTERY_NOTICE = 'POWER_ON_BATTERY_NOTICE',
  SUPPORT_TWO_CONNECTIONS = 'SUPPORT_TWO_CONNECTIONS',
  MULTIPLE_DEVICE_LIST = 'MULTIPLE_DEVICE_LIST'
}

export enum PromptLanguage {
  English = 'English',
  Chinese = 'Chinese'
}
