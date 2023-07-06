const useSelectSoundcoreDevice = () => {
   const result = await navigator.bluetooth.requestDevice({
        filters: [
            { manufacturerData: [{
                companyIdentifier: 
            }] },
        ]
   }); 
}