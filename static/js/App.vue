<template>
    <div>
        <section>
            <form action class="form-inline">
                <div class="form-group">
                    <label for="max-price">Maximum Price ($CAD)</label>
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
                    <span>{{ min_rating }}</span>
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
        </section>
        <wines v-bind:wine_color="wine_color" v-bind:wine_recommendations="wine_recommendations" />
    </div>
</template>

<script>
import axios from 'axios';
import Wines from './Wines.vue';

export default {
    data() {
        return {
            max_price: 20,
            min_rating: 14,
            wine_color: 'red',
            available_online: true,
            wines: []
        };
    },
    methods: {
        refresh_data: function(val, preVal) {
            axios
                .get('/wines/', {
                    params: {
                        max_price: this.max_price,
                        min_rating: this.min_rating,
                        available_online: this.available_online
                    }
                })
                .then(function(response) {
                    this.wines = JSON.parse(response.data).results;
                    console.log(this.wines);
                })
                .catch(function(error) {
                    console.log(error);
                });
        }
    },
    watch: {
        max_price: 'refresh_data',
        min_rating: 'refresh_data',
        available_online: 'refresh_data'
    },
    components: {
        Wines
    }
};
</script>