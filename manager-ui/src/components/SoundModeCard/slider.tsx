// import { Button, Grid, Icon, styled, Collapse, Stack, Slider as ValueSlider } from '@mui/material';
// import { useState } from 'react';
// import ANCIcon from '../../assets/ambient_icon_anc.png';
// import NormalIcon from '../../assets/ambient_icon_off.png';
// import TransIcon from '../../assets/ambient_icon_trans.png';
// import { ANCMode } from '@generated-types/soundcore-lib';

// export type SliderButton = {
//   label: string;
//   image: string;
//   subButtons: Array<{
//     label: string;
//     mode: string;
//     hasSlider?: boolean;
//     sliderValue?: number;
//   }>;
// };
// export type SliderLayout = {
//   left: {
//     label: string;
//     icon: string;
//     subButtons: Array<{
//       label: string;
//       mode: string;
//       hasSlider?: boolean;
//       sliderValue?: number;
//     }>;
//   };
//   center: {
//     label: string;
//     icon: string;
//     subButtons: Array<{
//       label: string;
//       mode: string;
//       hasSlider?: boolean;
//       sliderValue?: number;
//     }>;
//   };
//   right: {
//     label: string;
//     icon: string;
//     subButtons: Array<{
//       label: string;
//       mode: string;
//       hasSlider?: boolean;
//       sliderValue?: number;
//     }>;
//   };
// };

// export type SliderProps = {
//   layout: SliderLayout;
//   selectedValue: string;
//   onChange: (value: string) => void;
// };

// export const Slider: React.FC<SliderProps> = ({ layout, selectedValue, onChange }) => {
//   const keys = Object.keys(layout) as (keyof SliderLayout)[];
//   const currentSide = keys.find((key) =>
//     layout[key].subButtons.some((sub) => sub.mode === selectedValue)
//   );
//   const defaultPosition = currentSide || 'center';

//   const setPosition = (position: AllowedSliderPositions) => {
//     console.log('setPosition', position);
//   };
//   // const [position, setPosition] = useState<AllowedSliderPositions>(defaultPosition);
//   // const [icon, setIcon] = useState<string>(layout[defaultPosition].icon);

//   return (
//     <>
//       <Grid sx={{ paddingLeft: 0, justifyContent: 'center' }}>
//         <Grid item>
//           <SliderSelectorWrapper>
//             <SliderSelector position={defaultPosition}>
//               <Icon sx={{ display: 'flex', width: 32, height: 32, zIndex: 0 }}>
//                 <img src={icon} height="32" />
//               </Icon>
//             </SliderSelector>
//             <SliderButton
//               position="left"
//               icon={layout.left.icon}
//               setSliderIcon={setIcon}
//               setSliderPosition={setPosition}
//             />
//             <SliderButton
//               position="center"
//               icon={layout.center.icon}
//               setSliderIcon={setIcon}
//               setSliderPosition={setPosition}
//             />
//             <SliderButton
//               position="right"
//               icon={layout.right.icon}
//               setSliderIcon={setIcon}
//               setSliderPosition={setPosition}
//             />
//           </SliderSelectorWrapper>
//         </Grid>
//         <SliderSubButtons layout={layout} position={position} />
//       </Grid>
//     </>
//   );
// };

// const SliderSubButtons: React.FC<{
//   layout: SliderLayout;
//   position: AllowedSliderPositions;
// }> = ({ layout, position }) => {
//   const [isOpen, setOpen] = useState(layout[position].subButtons.length > 0);
//   // Search within the layout for the selected mode
//   const [currentValues, setCurrentValues] = useState<Record<string, string>>({});
//   console.log('currentValues', currentValues);

//   return (
//     <>
//       <Grid item sx={{ paddingTop: '0px !important' }}>
//         <Collapse in={isOpen}>
//           <SliderSubButtonsButtonGrid
//             layout={layout}
//             position={position}
//             setValue={(value) => {
//               setCurrentValues({ [position]: value });
//             }}
//             selectedMode={currentValues[position]}>
//             <></>
//             {/* <Collapse in={sliderPosition == 'left' && ancModeSelected.mode == 'AncCustomValue'}>
//               {ancCustomValue != null && (
//                 <ValueSlider
//                   size="small"
//                   value={ancCustomValue}
//                   onChange={(_, newValue) => setAncCustomValue(newValue)}
//                   onChangeCommitted={(_, newValue) => setAncCustomValue(newValue)}
//                   sx={{ mt: 2, pb: 0, width: '98%' }}
//                   min={0}
//                   max={10}
//                   marks
//                   aria-label="Small"
//                   valueLabelDisplay="auto"
//                 />
//               )}
//             </Collapse> */}
//           </SliderSubButtonsButtonGrid>
//         </Collapse>
//       </Grid>
//     </>
//   );
// };

// const SliderSubButtonsButtonGridButton = styled(Button, {
//   shouldForwardProp: (prop) => prop !== 'active'
// })<{ active?: boolean }>(({ theme, active }) => ({
//   //width: "100px",
//   backgroundColor: active ? theme.palette.primary.dark : 'transparent',
//   color: active ? theme.palette.text.primary : theme.palette.text.secondary
// }));

// const SliderSubButtonsButtonGrid: React.FC<{
//   layout: SliderLayout;
//   children: React.ReactNode;
//   setValue: React.Dispatch<string>;
//   selectedMode: string;
//   position: AllowedSliderPositions;
// }> = ({ layout, children, setValue, selectedMode, position }) => {
//   if (layout[position].subButtons.length === 0) {
//     return <></>;
//   }

//   return (
//     <Stack>
//       <Grid
//         container
//         direction="row"
//         spacing={1}
//         sx={{ display: 'flex', justifyContent: 'space-evenly', pt: 2 }}>
//         {layout[position].subButtons.map((button, idx) => (
//           <Grid item key={idx}>
//             <SliderSubButtonsButtonGridButton
//               active={button.mode === selectedMode}
//               onClick={() => {
//                 setValue(button.mode);
//               }}>
//               {button.label}
//             </SliderSubButtonsButtonGridButton>
//           </Grid>
//         ))}
//         {children}
//       </Grid>
//     </Stack>
//   );
// };
