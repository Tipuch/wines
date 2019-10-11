<template>
    <div class="wines">
        <ul class="nav nav-tabs">
            <li class="nav-item">
                <a
                    v-on:click="wine_color='red'"
                    class="nav-link"
                    v-bind:class="{ active: wine_color === 'red' }"
                    data-toggle="tab"
                >Red</a>
            </li>
            <li class="nav-item">
                <a
                    v-on:click="wine_color='white'"
                    class="nav-link"
                    v-bind:class="{ active: wine_color === 'white' }"
                    data-toggle="tab"
                >White</a>
            </li>
            <li class="nav-item">
                <a
                    v-on:click="wine_color='pink'"
                    class="nav-link"
                    v-bind:class="{ active: wine_color === 'pink' }"
                    data-toggle="tab"
                >Pink</a>
            </li>
        </ul>
        <div class="tab-content">
            <div
                class="tab-pane fade show"
                v-bind:class="{ active: wine_color === 'red' }"
                id="red-wines"
            >
                <table class="table table-striped">
                    <thead>
                        <tr>
                            <th scope="col">Name</th>
                            <th scope="col">Available Online</th>
                            <th scope="col">Country</th>
                            <th scope="col">Region</th>
                            <th scope="col">Designation of Origin</th>
                            <th scope="col">Producer</th>
                            <th scope="col">Volume</th>
                            <th scope="col">Price</th>
                            <th scope="col">Rating</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr
                            v-for="wine in wines.filter(wine => wine[7] === 'red')"
                            v-bind:key="wine[0]"
                        >
                            <td>{{ wine[1] }}</td>
                            <td>{{ wine[2] }}</td>
                            <td>{{ wine[3] }}</td>
                            <td>{{ wine[4] }}</td>
                            <td>{{ wine[5] }}</td>
                            <td>{{ wine[6] }}</td>
                            <td>{{ wine[8] }}</td>
                            <td>{{ wine[9] }}</td>
                            <td>{{ wine[10] }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div
                class="tab-pane fade show"
                v-bind:class="{ active: wine_color === 'white' }"
                id="white-wines"
            >
                <table class="table table-striped">
                    <thead>
                        <tr>
                            <th scope="col">Name</th>
                            <th scope="col">Available Online</th>
                            <th scope="col">Country</th>
                            <th scope="col">Region</th>
                            <th scope="col">Designation of Origin</th>
                            <th scope="col">Producer</th>
                            <th scope="col">Volume</th>
                            <th scope="col">Price</th>
                            <th scope="col">Rating</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr
                            v-for="wine in wines.filter(wine => wine[7] === 'white')"
                            v-bind:key="wine[0]"
                        >
                            <td>{{ wine[1] }}</td>
                            <td>{{ wine[2] }}</td>
                            <td>{{ wine[3] }}</td>
                            <td>{{ wine[4] }}</td>
                            <td>{{ wine[5] }}</td>
                            <td>{{ wine[6] }}</td>
                            <td>{{ wine[8] }}</td>
                            <td>{{ wine[9] }}</td>
                            <td>{{ wine[10] }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div
                class="tab-pane fade show"
                v-bind:class="{ active: wine_color === 'pink' }"
                id="pink-wines"
            >
                <table class="table table-striped">
                    <thead>
                        <tr>
                            <th scope="col">Name</th>
                            <th scope="col">Available Online</th>
                            <th scope="col">Country</th>
                            <th scope="col">Region</th>
                            <th scope="col">Designation of Origin</th>
                            <th scope="col">Producer</th>
                            <th scope="col">Volume</th>
                            <th scope="col">Price</th>
                            <th scope="col">Rating</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr
                            v-for="wine in wines.filter(wine => wine[7] === 'pink')"
                            v-bind:key="wine[0]"
                        >
                            <td>{{ wine[1] }}</td>
                            <td>{{ wine[2] }}</td>
                            <td>{{ wine[3] }}</td>
                            <td>{{ wine[4] }}</td>
                            <td>{{ wine[5] }}</td>
                            <td>{{ wine[6] }}</td>
                            <td>{{ wine[8] }}</td>
                            <td>{{ wine[9] }}</td>
                            <td>{{ wine[10] }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    props: ['wine_color', 'wines']
};
</script>