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
        <wines-table v-bind:wines="wines" />
    </div>
</template>

<script>
import axios from 'axios';
import WinesTable from './WinesTable.vue';

export default {
    data() {
        return {
            max_price: 20,
            min_rating: 14,
            available_online: true,
            wines: []
        }
    },
    methods: {
        refresh_data (val, preVal) {
            let self = this;
            axios
                .get('/wines/', {
                    params: {
                        max_price: this.max_price,
                        min_rating: this.min_rating,
                        available_online: this.available_online
                    }
                })
                .then(function(response) {
                    self.wines = response.data.results;
                })
                .catch(function(error) {
                    console.log(error);
                });
        }
    },
    watch: {
        max_price: 'refresh_data',
        min_rating: {
            handler: 'refresh_data',
            immediate: true
        },
        available_online: 'refresh_data'
    },
    components: {
        'wines-table': WinesTable
    }
};
</script>