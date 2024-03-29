import { AppBar as MUIBar, Box, Toolbar } from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import MoreVertIcon from '@mui/icons-material/MoreVert';

export default function AppBar() {
  return (
    <MUIBar sx={{ zIndex: -1 }}>
      <Toolbar sx={{ minHeight: 35, backgroundColor: '#202020' }} variant="dense">
        <Box sx={{ flexGrow: 1, display: { xs: 'flex', md: 'none' } }}>
          <ArrowBackIcon fontSize="medium" />
        </Box>
        <MoreVertIcon fontSize="medium" />
      </Toolbar>
    </MUIBar>
  );
}
