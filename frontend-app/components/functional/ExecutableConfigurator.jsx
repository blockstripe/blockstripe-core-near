import React, { useState } from 'react';

import { Button, Container, Typography } from '@mui/material';

import IncrementControl from '../common/IncrementControl';

const config = {
    month: {
        max: 12,
    },
    date: {
        max: 31,
    },
};

const ExecutableConfigurator = () => {
    const [executablePayload, setExecutablePayload] = useState({
        month: 1,
        date: 1,
    });

    const [executablePeriod, setExecutablePeriod] = useState('month');

    const handleSchedulePressed = () => {
        // TODO: Call contract method
    };

    const handleToggleIncrement = (direction, name) => {
        const executableConfig = config[name];

        if (
            (direction === '+' && (executablePayload[name] + 1) > executableConfig.max) ||
            (direction === '-' && executablePayload[name] === 1)
        ) {
            return;
        }

        setExecutablePayload(prev => ({
            ...prev,
            [name]: direction === '+' ? prev[name] + 1 : prev[name] - 1,
        }));
    };

    const handleSetPeriod = (newPeriod) => {
        setExecutablePeriod(newPeriod);
    };

    const _buildConfigurationForm = () => {
        return (
            <div className="executable-configurator-form">
                <p>Period of execution
                    {'\n'}<span
                        onClick={() => handleSetPeriod('month')}
                        className={executablePeriod === 'month' ? 'text-selectable text-selected' : ''}>month
                    </span>
                    {'\n'}<span
                        onClick={() => handleSetPeriod('year')}
                        className={executablePeriod === 'year' ? 'text-selectable text-selected' : ''}>year
                    </span>
                </p>
                <div>Execute payment every
                    <IncrementControl handleToggleIncrement={handleToggleIncrement} value={executablePayload.month} name="month" />
                month</div>
                <div>at the
                    <IncrementControl handleToggleIncrement={handleToggleIncrement} value={executablePayload.date} name="date" />
                date</div>
            </div>
        );
    };
     
    return (
        <Container className="text-align-center">
            <Typography variant="h2">Schedule new reccuring payment</Typography>
            <br />
            <form className="mr-top-20">
                {_buildConfigurationForm()}
            </form>
            <br />
            <Button onClick={handleSchedulePressed} variant="contained">Schedule</Button>
        </Container>
    );
};

export default ExecutableConfigurator;
