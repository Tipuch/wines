<template>
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
                        <label class="custom-control-label" for="available-online">Available Online</label>
                    </div>
                </form>
                <p>The results are filtered by $ per ml in ascending order.</p>
            </div>
        </section>
        <wines-table v-bind:wines="wines" />
    </div>
</template>

<script>
import axios from 'axios';
import _ from 'lodash';
import WinesTable from './WinesTable.vue';

export default {
    data() {
        return {
            max_price: 20,
            min_rating: 14,
            available_online: true,
            wines: []
        };
    },
    methods: {
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
                    this.wines = response.data.results;
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