import { Badge } from '@mui/material';

const IncrementControl = ({ handleToggleIncrement, value, name }) => {
    return (
        <div className="increment-control__container">
            <div className="increment-control__control-button" onClick={() => handleToggleIncrement('-', name)}>
                <Badge badgeContent={'-'} color="secondary" />
            </div>
            <div className="increment-control__control-button">
                <Badge badgeContent={value?.toString()} color="secondary" />
            </div>
            <div className="increment-control__control-button" onClick={() => handleToggleIncrement('+', name)}>
                <Badge badgeContent={'+'} color="secondary" />
            </div>
        </div>
    );
};

export default IncrementControl;
