<template>
    <v-app>
        <div>
            <section>
                <div class="filters">
                    <form onsubmit="return false;" class="form-inline">
                        <div class="form-group">
                            <label for="max-price">Maximum Price ($/750 ml)</label>
                            <input
                                id="max-price"
                                type="number"
                                min="0"
                                class="form-control"
                                placeholder="20.00"
                                v-model="max_price"
                            />
                        </div>
                        <div class="form-group">
                            <label for="min-rating">Minimum Rating:</label>
                            <span>{{ min_rating }}/20</span>
                            <input
                                type="range"
                                class="custom-range"
                                min="1"
                                max="20"
                                id="min-rating"
                                v-model="min_rating"
                            />
                        </div>
                        <div class="custom-control custom-checkbox">
                            <input
                                type="checkbox"
                                id="available-online"
                                name="available-online"
                                class="custom-control-input"
                                v-model="available_online"
                            />
                            <label class="custom-control-label" for="available-online"
                                >Available Online</label
                            >
                        </div>
                    </form>
                    <p>The results are ordered by $ per ml in ascending order.</p>
                </div>
            </section>
            <wines-table
                v-bind:redWines="wines.filter(wine => wine.color === 'red')"
                v-bind:whiteWines="wines.filter(wine => wine.color === 'white')"
                v-bind:pinkWines="wines.filter(wine => wine.color === 'pink')"
            />
        </div>
    </v-app>
</template>

<script>
import axios from 'axios';
import _ from 'lodash';
import WinesTable from './WinesTable.vue';

export default {
    data() {
        return {
            max_price: null,
            min_rating: 14,
            available_online: true,
            wines: []
        };
    },
    methods: {
        getPrice(volumeStr, priceStr) {
            let volume = parseInt(volumeStr);
            if (volume === 750) {
                return priceStr;
            } else {
                let price = parseFloat(priceStr.substring(1));
                const pricePer750Ml = (price / volume) * 750;
                return `${priceStr} ($${pricePer750Ml.toFixed(2)}/750ml)`;
            }
        },

        refresh_data() {
            let data = {
                min_rating: this.min_rating,
                available_online: this.available_online
            };
            if (this.max_price) {
                data['max_price'] = this.max_price;
            }

            axios
                .get('/wines/', {
                    params: data
                })
                .then(response => {
                    this.wines = response.data.results.map(wine => {
                        return {
                            id: wine[0],
                            name: wine[1],
                            availableOnline: wine[2] ? 'yes' : 'no',
                            country: wine[3],
                            region: wine[4],
                            designationOfOrigin: wine[5],
                            producer: wine[6],
                            color: wine[7],
                            volume: wine[8],
                            price: this.getPrice(wine[8], wine[9]),
                            rating: wine[10]
                        };
                    });
                })
                .catch(error => {
                    console.log(error);
                });
        },
        safe_refresh_data(val, pre_val) {
            this.debounce_refresh_data();
        }
    },
    created: function() {
        // _.debounce is a function provided by lodash to limit how
        // often a particularly expensive operation can be run.
        // In this case, we want to limit how often we access
        // yesno.wtf/api, waiting until the user has completely
        // finished typing before making the ajax request. To learn
        // more about the _.debounce function (and its cousin
        // _.throttle), visit: https://lodash.com/docs#debounce
        this.debounce_refresh_data = _.debounce(this.refresh_data, 500);
        this.debounce_refresh_data();
    },
    watch: {
        max_price: 'safe_refresh_data',
        min_rating: 'safe_refresh_data',
        available_online: 'safe_refresh_data'
    },
    components: {
        'wines-table': WinesTable
    }
};
</script>
