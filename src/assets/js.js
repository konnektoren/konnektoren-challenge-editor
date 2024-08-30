document.getElementById('greet-button')
    .addEventListener('click', function() {
            console.log(challenge);

            const optionsList = document.getElementById('options-list');
            optionsList.innerHTML = '';

            if (challenge.data && Array.isArray(challenge.data.options)) {
                challenge.data.options.forEach(option => {
                    const listItem = document.createElement('li');
                    listItem.textContent = option.name;
                    optionsList.appendChild(listItem);
                });
            } else {
                optionsList.textContent = 'No options available';
            }
        }
    );
